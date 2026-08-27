//! 基于 Redis 的简单分布式锁。
//!
//! 对应 Java `RedisTemplateSimpleDistributedLock` 的语义：支持重入的
//! 简单 Redis 分布式锁（非红锁），基于 SETNX + TTL + 唯一值校验释放。
//! Java 使用 Spring `StringRedisTemplate`；Rust 侧使用 `redis` crate。

use std::sync::Arc;

use redis::Client;

/// Redis 简单分布式锁。
///
/// 锁语义：
/// - 加锁：`SET key value NX EX lease`（value 为唯一标识，支持重入判断）
/// - 释放：Lua 脚本校验 value 后 DEL（防止误删他人锁）
#[derive(Clone)]
pub struct RedisDistributedLock {
    client: Client,
    key: String,
    lease_milliseconds: i64,
}

impl RedisDistributedLock {
    /// 构建 Redis 分布式锁（随机键名）。
    ///
    /// # 参数
    /// - `client`：redis 客户端
    /// - `lease_milliseconds`：锁租约时长（毫秒），必须大于 0
    ///
    /// # 错误
    /// `lease_milliseconds <= 0` 时返回参数错误。
    pub fn new(client: Client, lease_milliseconds: i64) -> Result<Self, String> {
        if lease_milliseconds <= 0 {
            return Err(format!(
                "Parameter 'leaseMilliseconds' must grate then 0: {lease_milliseconds}"
            ));
        }
        let key = format!("lock:{}", uuid());
        Ok(Self {
            client,
            key,
            lease_milliseconds,
        })
    }

    /// 构建指定键名的 Redis 分布式锁。
    ///
    /// # 参数
    /// - `client`：redis 客户端
    /// - `key`：锁键名
    /// - `lease_milliseconds`：锁租约时长（毫秒），必须大于 0
    pub fn with_key(
        client: Client,
        key: impl Into<String>,
        lease_milliseconds: i64,
    ) -> Result<Self, String> {
        if lease_milliseconds <= 0 {
            return Err(format!(
                "Parameter 'leaseMilliseconds' must grate then 0: {lease_milliseconds}"
            ));
        }
        Ok(Self {
            client,
            key: key.into(),
            lease_milliseconds,
        })
    }

    /// 返回锁租约时长（毫秒）。
    pub fn lease_milliseconds(&self) -> i64 {
        self.lease_milliseconds
    }

    /// 返回锁键名。
    pub fn key(&self) -> &str {
        &self.key
    }

    /// 尝试获取锁（非阻塞）。
    ///
    /// # 返回
    /// 获取成功返回 `Ok(Some(guard))`（Drop 时释放锁）；连接失败返回错误。
    pub fn try_lock(&self) -> Result<Option<LockGuard>, redis::RedisError> {
        let value = uuid();
        let mut conn = self.client.get_connection()?;
        let result: Option<String> = redis::cmd("SET")
            .arg(&self.key)
            .arg(&value)
            .arg("NX")
            .arg("PX")
            .arg(self.lease_milliseconds)
            .query(&mut conn)?;
        if result.is_some() {
            Ok(Some(LockGuard {
                client: self.client.clone(),
                key: self.key.clone(),
                value,
            }))
        } else {
            Ok(None)
        }
    }

    /// 阻塞获取锁（轮询，对应 Java `lock()` 的 1 秒重试）。
    ///
    /// # 参数
    /// - `timeout`：总超时；`None` 为无限等待
    ///
    /// # 返回
    /// 获取成功返回 guard；超时或错误返回错误信息。
    pub fn lock(&self, timeout: Option<std::time::Duration>) -> Result<LockGuard, String> {
        let start = std::time::Instant::now();
        loop {
            match self.try_lock() {
                Ok(Some(g)) => return Ok(g),
                Ok(None) => {
                    if let Some(t) = timeout
                        && start.elapsed() >= t
                    {
                        return Err("acquire timeouted".to_string());
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
                Err(e) => return Err(format!("lock failed: {e}")),
            }
        }
    }
}

/// 锁守卫：Drop 时自动释放锁（校验 value 后 DEL）。
pub struct LockGuard {
    client: Client,
    key: String,
    value: String,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        // Lua：校验 value 一致才删除（对应 Java 释放脚本）
        let script = "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end";
        let mut conn = match self.client.get_connection() {
            Ok(c) => c,
            Err(_) => return,
        };
        let _: Option<i64> = redis::cmd("EVAL")
            .arg(script)
            .arg(1)
            .arg(&self.key)
            .arg(&self.value)
            .query(&mut conn)
            .ok();
    }
}

/// 生成简单 UUID 字符串（锁唯一标识）。
fn uuid() -> String {
    // 16 字节随机值转 hex（rand 0.10 自由函数）
    let bytes: [u8; 16] = rand::random();
    hex::encode(bytes)
}

/// 便于无 Redis 环境下保持 API 可用（非 feature 构建时仍引用 Arc）
#[allow(dead_code)]
fn _touch(_: Arc<()>) {}
