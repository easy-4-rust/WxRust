//! 公共 API 抽象。
//!
//! 对应 Java `me.chanjar.weixin.common.api` 包：
//! 常量（[`wx_consts`]）、异常处理回调（[`WxErrorExceptionHandler`]）、
//! 重复消息检查器（内存/Redis 实现）。

pub mod wx_consts;
pub mod wx_error_exception_handler;
pub mod wx_message_duplicate_checker;
pub mod wx_message_in_memory_duplicate_checker;
pub mod wx_message_in_memory_duplicate_checker_singleton;
#[cfg(feature = "redis")]
pub mod wx_message_in_redis_duplicate_checker;

pub use wx_error_exception_handler::WxErrorExceptionHandler;
pub use wx_message_duplicate_checker::WxMessageDuplicateChecker;
pub use wx_message_in_memory_duplicate_checker::WxMessageInMemoryDuplicateChecker;
pub use wx_message_in_memory_duplicate_checker_singleton::WxMessageInMemoryDuplicateCheckerSingleton;
#[cfg(feature = "redis")]
pub use wx_message_in_redis_duplicate_checker::WxMessageInRedisDuplicateChecker;
