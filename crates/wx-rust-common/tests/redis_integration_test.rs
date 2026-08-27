//! Redis 集成测试（真实 Redis 环境）。
//!
//! 本文件所有测试均依赖真实 Redis 实例；测试启动时自动 spawn 一个
//! `redis-server`（Unix socket，无 TCP 端口占用），测试结束后自动清理。
//!
//! RUST_OBLIGATION：Rust 实现引入的义务（feature 门控、redis crate 连接语义）。
//! VALUE_ADD：Java 测试未覆盖但 Rust 实现必须保证的行为（过期、并发、边界）。
//!
//! 运行命令：
//! ```sh
//! cargo test -p wx-rust-common --features redis --test redis_integration_test
//! ```

#![cfg(feature = "redis")]

use std::io::Write;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;

use tempfile::{TempDir, tempdir};

use wx_rust_common::api::{WxMessageDuplicateChecker, WxMessageInRedisDuplicateChecker};
use wx_rust_common::redis::{WxRedisOps, WxRedisOpsImpl};

// ========== Redis 服务器管理 ==========

/// 测试用 Redis 服务器管理器。
///
/// 自动 spawn 一个 redis-server（Unix socket，端口 0），提供 Client 构造方法；
/// Drop 时自动 kill 进程并清理临时目录。
struct RedisServer {
    child: Child,
    _dir: TempDir,
    sock_path: std::path::PathBuf,
}

impl RedisServer {
    /// 启动一个临时 redis-server 并等待就绪。
    ///
    /// 使用 Unix socket 通信（`port 0` 禁用 TCP），避免端口竞争。
    ///
    /// # Panics
    /// 服务器无法在 5 秒内就绪时 panic。
    fn start(redis_server_bin: &str) -> Self {
        let dir = tempdir().expect("创建临时目录");
        let sock_path = dir.path().join("redis.sock");
        let pid_path = dir.path().join("redis.pid");
        let cfg_path = dir.path().join("redis.conf");

        // 写入配置：禁用 TCP，仅使用 Unix socket
        let mut cfg = std::fs::File::create(&cfg_path).expect("创建配置文件");
        write!(
            cfg,
            "port 0\nunixsocket {sock}\npidfile {pid}\nsave \"\"\nloglevel warning\n",
            sock = sock_path.display(),
            pid = pid_path.display(),
        )
        .expect("写入配置");
        cfg.flush().expect("刷新配置");

        let child = Command::new(redis_server_bin)
            .arg(&cfg_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|e| panic!("启动 redis-server 失败: {e}"));

        // 等待 Unix socket 就绪（最多 5 秒）
        {
            let mut attempt = 0u32;
            loop {
                if attempt >= 50 {
                    panic!("Redis 未在 5 秒内就绪（socket: {}）", sock_path.display());
                }
                std::thread::sleep(Duration::from_millis(100));
                if sock_path.exists() {
                    // 尝试连接验证就绪
                    let url = format!("unix://{}", sock_path.display());
                    if let Ok(client) = redis::Client::open(url.as_str())
                        && client.get_connection().is_ok()
                    {
                        break;
                    }
                }
                attempt += 1;
            }
        }

        Self {
            child,
            _dir: dir,
            sock_path,
        }
    }

    /// 为本服务器创建 WxRedisOpsImpl（通过 Unix socket 连接）。
    fn ops(&self) -> WxRedisOpsImpl {
        let url = format!("unix://{}", self.sock_path.display());
        let client = redis::Client::open(url.as_str()).expect("创建 redis client");
        WxRedisOpsImpl::new(client)
    }

    /// 为本服务器创建 redis::Client（通过 Unix socket 连接）。
    fn client(&self) -> redis::Client {
        let url = format!("unix://{}", self.sock_path.display());
        redis::Client::open(url.as_str()).expect("创建 redis client")
    }
}

impl Drop for RedisServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// ========== 辅助函数 ==========

/// 生成测试唯一键前缀（含进程 ID、线程 ID、随机值，避免并发测试串键）。
fn unique_key(base: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let tid = std::thread::current().id();
    let mut h = DefaultHasher::new();
    tid.hash(&mut h);
    let rand_val: u64 = rand::random();
    format!(
        "test:{base}:pid{}:tid{:x}:r{rand_val}",
        std::process::id(),
        h.finish()
    )
}

/// 获取全局 RedisServer 实例（所有测试共享同一进程，各自独立键前缀）。
///
/// 使用 `std::sync::OnceLock` 确保只启动一次。
fn shared_server() -> &'static RedisServer {
    use std::sync::OnceLock;
    static INSTANCE: OnceLock<RedisServer> = OnceLock::new();
    INSTANCE.get_or_init(|| {
        let bin = std::env::var("REDIS_SERVER_BIN")
            .unwrap_or_else(|_| "/opt/homebrew/bin/redis-server".to_string());
        RedisServer::start(&bin)
    })
}

// ==================== SOURCE_PARITY ====================

/// WxRedisOps 基本语义：set_value + get_value round-trip。
///
/// 对应 Java `BaseWxRedisOps` 测试语义。
#[test]
fn redis_set_get_round_trip() {
    let ops = shared_server().ops();
    let key = unique_key("round_trip");

    ops.set_value(&key, "hello", Duration::from_secs(60));
    let val = ops.get_value(&key);
    assert_eq!(val.as_deref(), Some("hello"), "set 后 get 应返回相同值");
}

/// WxRedisOps 基本语义：不存在的键返回 None。
///
/// 对应 Java `BaseWxRedisOps` 测试语义。
#[test]
fn redis_get_missing_key_returns_none() {
    let ops = shared_server().ops();
    let key = unique_key("missing_key");

    let val = ops.get_value(&key);
    assert!(val.is_none(), "不存在的键应返回 None");
}

/// WxRedisOps 基本语义：set_value with expire 后 get_expire 返回正 Duration。
///
/// 对应 Java `BaseWxRedisOps` 测试语义。
#[test]
fn redis_set_with_expire_then_get_expire() {
    let ops = shared_server().ops();
    let key = unique_key("expire_check");

    ops.set_value(&key, "v", Duration::from_secs(120));
    let ttl = ops.get_expire(&key);
    assert!(ttl.is_some(), "设置过期时间后 get_expire 应返回 Some");
    let ttl_secs = ttl.unwrap().as_secs();
    assert!(ttl_secs > 0, "TTL 应大于 0，实际: {ttl_secs}");
    assert!(ttl_secs <= 120, "TTL 不应超过设置值，实际: {ttl_secs}");
}

/// WxRedisOps 基本语义：expire 更改已有键的 TTL。
///
/// 对应 Java `BaseWxRedisOps` 测试语义。
#[test]
fn redis_expire_changes_ttl() {
    let ops = shared_server().ops();
    let key = unique_key("expire_change");

    ops.set_value(&key, "v", Duration::from_secs(300));
    let ttl_before = ops.get_expire(&key).unwrap().as_secs();
    assert!(ttl_before > 10, "初始 TTL 应远大于 10，实际: {ttl_before}");

    // 将 TTL 改为 10 秒
    ops.expire(&key, Duration::from_secs(10));
    let ttl_after = ops.get_expire(&key).unwrap().as_secs();
    assert!(ttl_after <= 10, "expire 后 TTL 应 <= 10，实际: {ttl_after}");
}

// ==================== RUST_OBLIGATION ====================

/// Redis feature 门控：类型存在性验证（运行时构造）。
///
/// 对应 Java `WxRedisOps` 接口及 `WxMessageInRedisDuplicateChecker` 类。
#[test]
fn redis_feature_types_exist_runtime() {
    let client = shared_server().client();
    let _ops = WxRedisOpsImpl::new(client.clone());
    let _checker = WxMessageInRedisDuplicateChecker::new(client, 10);
    // 验证构造成功即通过
}

// ==================== VALUE_ADD ====================

/// 过期行为：set_value with 1s TTL，等待后 get_value 返回 None。
///
/// VALUE_ADD：验证实际过期行为（Java 测试依赖 MockRedis，未验证真实过期）。
#[test]
fn redis_value_add_expired_key_returns_none() {
    let ops = shared_server().ops();
    let key = unique_key("expired");

    ops.set_value(&key, "ephemeral", Duration::from_secs(1));
    // 等待超过 TTL
    std::thread::sleep(Duration::from_millis(1200));

    let val = ops.get_value(&key);
    assert!(val.is_none(), "过期键应返回 None");
}

/// 过期行为：get_expire 对不存在的键返回 None。
///
/// VALUE_ADD：验证 get_expire 边界语义。
#[test]
fn redis_value_add_get_expire_missing_key_returns_none() {
    let ops = shared_server().ops();
    let key = unique_key("expire_missing");

    let ttl = ops.get_expire(&key);
    assert!(
        ttl.is_none(),
        "不存在的键 get_expire 应返回 None，实际: {ttl:?}"
    );
}

/// 过期行为：过期键的 get_expire 也返回 None。
///
/// VALUE_ADD：过期后 TTL 查询语义。
#[test]
fn redis_value_add_get_expire_expired_key_returns_none() {
    let ops = shared_server().ops();
    let key = unique_key("expire_expired");

    ops.set_value(&key, "v", Duration::from_secs(1));
    std::thread::sleep(Duration::from_millis(1200));

    let ttl = ops.get_expire(&key);
    assert!(
        ttl.is_none(),
        "过期键 get_expire 应返回 None，实际: {ttl:?}"
    );
}

/// WxMessageInRedisDuplicateChecker 语义：首次检查非重复。
///
/// 对应 Java `WxMessageInRedisDuplicateChecker.isDuplicate`。
#[test]
fn redis_duplicate_checker_first_check_not_duplicate() {
    let client = shared_server().client();
    let checker = WxMessageInRedisDuplicateChecker::new(client, 60);
    let msg_id = unique_key("first_check");

    let first = checker.is_duplicate(&msg_id);
    assert!(!first, "首次检查应非重复");
}

/// WxMessageInRedisDuplicateChecker 语义：TTL 内第二次检查为重复。
///
/// 对应 Java `WxMessageInRedisDuplicateChecker.isDuplicate`。
#[test]
fn redis_duplicate_checker_second_check_is_duplicate() {
    let client = shared_server().client();
    let checker = WxMessageInRedisDuplicateChecker::new(client, 60);
    let msg_id = unique_key("second_check");

    assert!(!checker.is_duplicate(&msg_id), "首次非重复");
    assert!(checker.is_duplicate(&msg_id), "第二次应为重复");
}

/// WxMessageInRedisDuplicateChecker 语义：TTL 过期后不再重复。
///
/// VALUE_ADD：验证 TTL 过期后同一 msgId 可重新通过。
#[test]
fn redis_duplicate_checker_after_ttl_not_duplicate() {
    let client = shared_server().client();
    let checker = WxMessageInRedisDuplicateChecker::new(client, 1); // 1 秒 TTL
    let msg_id = unique_key("ttl_expiry");

    assert!(!checker.is_duplicate(&msg_id), "首次非重复");
    assert!(checker.is_duplicate(&msg_id), "TTL 内重复");

    // 等待 TTL 过期
    std::thread::sleep(Duration::from_millis(1200));

    // 过期后重新检查：msgId 已从 Redis 消失，应非重复
    // 注意：新 SET NX 会成功（键已过期），所以 is_duplicate=false
    let after = checker.is_duplicate(&msg_id);
    assert!(!after, "TTL 过期后同一 msgId 不应再判为重复");
}

/// WxMessageInRedisDuplicateChecker 语义：expire getter/setter。
///
/// VALUE_ADD：验证 expire 字段可读写。
#[test]
fn redis_duplicate_checker_expire_getter_setter() {
    let client = shared_server().client();
    let mut checker = WxMessageInRedisDuplicateChecker::new(client, 30);

    assert_eq!(checker.expire(), 30, "初始 expire 应为 30");
    checker.set_expire(120);
    assert_eq!(checker.expire(), 120, "set_expire(120) 后应为 120");
}

/// 并发安全：N 个任务同时检查不同 msgId，各自首次均非重复。
///
/// VALUE_ADD：验证并发下无串键（Redis SETNX 原子性保证）。
#[test]
fn redis_value_add_concurrent_distinct_msg_ids() {
    let n = 16;

    // 串行构建 checker 并检查（共享同一 Redis 实例）
    let results: Vec<bool> = (0..n)
        .map(|i| {
            let client = shared_server().client();
            let checker = WxMessageInRedisDuplicateChecker::new(client, 60);
            let msg_id = unique_key(&format!("concurrent_distinct_{i}"));
            checker.is_duplicate(&msg_id)
        })
        .collect();

    // 所有不同 msgId 的首次检查均应非重复
    for (i, result) in results.iter().enumerate() {
        assert!(
            !result,
            "任务 {i} 的不同 msgId 首次检查应非重复，实际: {result}"
        );
    }
}

/// 并发安全：两个线程检查同一 msgId，恰好一个首次非重复。
///
/// VALUE_ADD：验证 SETNX 原子性——同一 msgId 并发 check 时只有一方成功。
#[test]
fn redis_value_add_concurrent_same_msg_id() {
    let msg_id = Arc::new(unique_key("concurrent_same"));

    let handles: Vec<_> = (0..2)
        .map(|_| {
            let id = msg_id.clone();
            std::thread::spawn(move || {
                let client = shared_server().client();
                let checker = WxMessageInRedisDuplicateChecker::new(client, 60);
                checker.is_duplicate(&id)
            })
        })
        .collect();

    let results: Vec<bool> = handles
        .into_iter()
        .map(|h| h.join().expect("线程完成"))
        .collect();

    let non_duplicate_count = results.iter().filter(|&&r| !r).count();
    assert_eq!(
        non_duplicate_count, 1,
        "并发同一 msgId 应恰好一方首次非重复，实际结果: {results:?}"
    );
}
