//! IVR 呼叫对象。
//!
//! 对应 Java `me.chanjar.weixin.qidian.bean.dial` 包。

pub mod ivr;
pub mod ivr_dial_request;
pub mod ivr_dial_response;
pub mod ivr_list_response;

pub use ivr::Ivr;
pub use ivr_dial_request::IVRDialRequest;
pub use ivr_dial_response::IVRDialResponse;
pub use ivr_list_response::IVRListResponse;
