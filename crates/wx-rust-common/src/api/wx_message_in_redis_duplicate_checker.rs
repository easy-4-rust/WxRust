//! 基于 Redis 的消息重复检查器。
//!
//! 对应 Java `me.chanjar.weixin.common.api.WxMessageInRedisDuplicateChecker`
//! （feature `redis` 门控）。

use super::WxMessageDuplicateChecker;

/// 基于 Redis 的消息重复检查器。
///
/// 利用 Redis 的 SETNX 语义：对消息 ID 执行"设置成功则非重复"判断，
/// 设置失败说明该消息 ID 已存在（重复消息）。键名前缀 `wx:message:duplicate:check:`。
///
/// # 注意
/// Java 使用 Redisson 的 `RBucket.trySet`；Rust 侧使用 `redis` crate 的 `SET NX EX`。
#[derive(Debug, Clone)]
pub struct WxMessageInRedisDuplicateChecker {
    /// Redis 连接池（多线程安全）
    client: redis::Client,

    /// 键过期时间（秒），默认 10
    expire: u32,
}

impl WxMessageInRedisDuplicateChecker {
    /// 构建 Redis 重复检查器。
    ///
    /// # 参数
    /// - `client`：redis 客户端（`redis::Client`）
    /// - `expire`：键过期时间（秒）
    ///
    /// # 返回
    /// 构建失败时返回错误
    pub fn new(client: redis::Client, expire: u32) -> Self {
        Self { client, expire }
    }

    /// 返回键过期时间（秒）。
    pub fn expire(&self) -> u32 {
        self.expire
    }

    /// 设置键过期时间（秒）。
    pub fn set_expire(&mut self, expire: u32) {
        self.expire = expire;
    }

    /// 尝试设置键（SETNX 语义）。
    fn try_set(&self, key: &str) -> Result<bool, redis::RedisError> {
        let mut conn = self.client.get_connection()?;
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg("1")
            .arg("NX")
            .arg("EX")
            .arg(self.expire)
            .query(&mut conn)?;
        Ok(result.is_some())
    }
}

impl WxMessageDuplicateChecker for WxMessageInRedisDuplicateChecker {
    /// 判断消息是否重复：SET NX 成功（键不存在）为非重复，失败为重复。
    fn is_duplicate(&self, message_id: &str) -> bool {
        let key = format!("wx:message:duplicate:check:{message_id}");
        match self.try_set(&key) {
            Ok(set_success) => !set_success,
            Err(e) => {
                tracing::error!("redis 检查消息重复失败: {e}");
                // Java 侧 trySet 异常会向上抛；这里保守返回非重复并记录错误
                false
            }
        }
    }
}
