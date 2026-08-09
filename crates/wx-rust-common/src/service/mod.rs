//! 服务接口。
//!
//! 对应 Java `me.chanjar.weixin.common.service` 包。

pub mod wx_img_proc_service;
pub mod wx_oauth2_service;
pub mod wx_ocr_service;
pub mod wx_service;

pub use wx_img_proc_service::WxImgProcService;
pub use wx_oauth2_service::WxOAuth2Service;
pub use wx_ocr_service::WxOcrService;
pub use wx_service::WxService;
