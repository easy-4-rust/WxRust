//! G1 组（核心服务组）子服务 trait 注册。
//!
//! 本组实现 Java `cn.binarywang.wx.miniapp.api` 中 9 个核心子服务接口：
//! WxMaUserService/WxMaMsgService/WxMaMediaService/WxMaKefuService/
//! WxMaAnalysisService/WxMaCodeService/WxMaExpressService/WxMaSecurityService/
//! WxMaSettingService。由 Wave 3 统一装配到门面。

//! 模块文件位于 `api/` 根目录（`wx_ma_<域>_service.rs`，与任务文件布局
//! 一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式
//! 指回根目录文件。

#[path = "wx_ma_analysis_service.rs"]
pub mod wx_ma_analysis_service;
#[path = "wx_ma_code_service.rs"]
pub mod wx_ma_code_service;
#[path = "wx_ma_express_service.rs"]
pub mod wx_ma_express_service;
#[path = "wx_ma_kefu_service.rs"]
pub mod wx_ma_kefu_service;
#[path = "wx_ma_media_service.rs"]
pub mod wx_ma_media_service;
#[path = "wx_ma_msg_service.rs"]
pub mod wx_ma_msg_service;
#[path = "wx_ma_security_service.rs"]
pub mod wx_ma_security_service;
#[path = "wx_ma_setting_service.rs"]
pub mod wx_ma_setting_service;
#[path = "wx_ma_user_service.rs"]
pub mod wx_ma_user_service;

pub use wx_ma_analysis_service::WxMaAnalysisService;
pub use wx_ma_code_service::WxMaCodeService;
pub use wx_ma_express_service::WxMaExpressService;
pub use wx_ma_kefu_service::WxMaKefuService;
pub use wx_ma_media_service::WxMaMediaService;
pub use wx_ma_msg_service::WxMaMsgService;
pub use wx_ma_security_service::WxMaSecurityService;
pub use wx_ma_setting_service::WxMaSettingService;
pub use wx_ma_user_service::WxMaUserService;
