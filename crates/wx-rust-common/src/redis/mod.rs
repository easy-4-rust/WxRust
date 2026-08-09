//! Redis 操作抽象。
//!
//! 对应 Java `me.chanjar.weixin.common.redis` 包（`WxRedisOps` 接口及
//! Jedis/Redisson/RedisTemplate 三实现，feature `redis` 门控）。
//! Rust 侧以 `redis` crate 统一承载，提供与 `WxRedisOps` 语义对齐的 trait。

#[cfg(feature = "redis")]
pub mod wx_redis_ops;

#[cfg(feature = "redis")]
pub use wx_redis_ops::{WxRedisOps, WxRedisOpsImpl};
