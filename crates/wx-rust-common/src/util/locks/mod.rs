//! 分布式锁。
//!
//! 对应 Java `me.chanjar.weixin.common.util.locks` 包。
//! Java 提供基于 Jedis/RedisTemplate 的锁实现（前者已 `@Deprecated`）；
//! Rust 侧由 [`RedisDistributedLock`]（`redis` crate，feature 门控）统一提供。

#[cfg(feature = "redis")]
pub mod redis_distributed_lock;

#[cfg(feature = "redis")]
pub use redis_distributed_lock::RedisDistributedLock;
