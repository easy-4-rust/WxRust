//! 微信 Redis 操作。
//!
//! 对应 Java `me.chanjar.weixin.common.redis.WxRedisOps`（及 BaseWxRedisOps）。

use std::time::Duration;

use redis::Client;

/// 微信 Redis 相关操作。
///
/// 该接口不承诺稳定（与 Java 注释一致）；外部实现建议继承默认实现
/// [`WxRedisOpsImpl`]。
pub trait WxRedisOps: Send + Sync {
    /// 获取键值。
    ///
    /// # 参数
    /// - `key`：键
    ///
    /// # 返回
    /// 值；不存在或错误时返回 `None`。
    fn get_value(&self, key: &str) -> Option<String>;

    /// 设置键值并指定过期时间。
    ///
    /// # 参数
    /// - `key`：键
    /// - `value`：值
    /// - `expire`：过期时长
    fn set_value(&self, key: &str, value: &str, expire: Duration);

    /// 获取键剩余过期时间。
    ///
    /// # 参数
    /// - `key`：键
    ///
    /// # 返回
    /// 剩余过期时长；不存在时返回 `None`。
    fn get_expire(&self, key: &str) -> Option<Duration>;

    /// 设置键过期时间。
    ///
    /// # 参数
    /// - `key`：键
    /// - `expire`：过期时长
    fn expire(&self, key: &str, expire: Duration);
}

/// 基于 `redis` crate 的 `WxRedisOps` 实现（对应 Java `RedissonWxRedisOps` 语义）。
#[derive(Debug, Clone)]
pub struct WxRedisOpsImpl {
    client: Client,
}

impl WxRedisOpsImpl {
    /// 构建 Redis 操作实现。
    ///
    /// # 参数
    /// - `client`：redis 客户端
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl WxRedisOps for WxRedisOpsImpl {
    fn get_value(&self, key: &str) -> Option<String> {
        let mut conn = self.client.get_connection().ok()?;
        redis::cmd("GET").arg(key).query(&mut conn).ok()
    }

    fn set_value(&self, key: &str, value: &str, expire: Duration) {
        if let Ok(mut conn) = self.client.get_connection() {
            let _: Result<(), _> = redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX")
                .arg(expire.as_secs())
                .query(&mut conn);
        }
    }

    fn get_expire(&self, key: &str) -> Option<Duration> {
        let mut conn = self.client.get_connection().ok()?;
        let secs: i64 = redis::cmd("TTL").arg(key).query(&mut conn).ok()?;
        if secs < 0 {
            return None;
        }
        Some(Duration::from_secs(secs as u64))
    }

    fn expire(&self, key: &str, expire: Duration) {
        if let Ok(mut conn) = self.client.get_connection() {
            let _: Result<(), _> = redis::cmd("EXPIRE")
                .arg(key)
                .arg(expire.as_secs())
                .query(&mut conn);
        }
    }
}
