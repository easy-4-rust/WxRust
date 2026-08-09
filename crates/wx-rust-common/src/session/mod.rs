//! 会话管理。
//!
//! 对应 Java `me.chanjar.weixin.common.session` 包。
//! Java 的 StandardSession/StandardSessionFacade/InternalSession/Constants/
//! TooManyActiveSessionsException/LocalStrings.properties 在 Rust 中由
//! [`StandardSession`] + [`StandardSessionManager`] 承载（语义对齐，类型合并）。

pub mod standard_session;
pub mod standard_session_manager;
pub mod wx_session;
pub mod wx_session_manager;

pub use standard_session::StandardSession;
pub use standard_session_manager::StandardSessionManager;
pub use wx_session::WxSession;
pub use wx_session_manager::WxSessionManager;
