//! G1 组（核心服务组）子服务实现注册。
//!
//! 本组实现 Java `cn.binarywang.wx.miniapp.api.impl` 中 9 个核心子服务：
//! WxMaUserServiceImpl/WxMaMsgServiceImpl/WxMaMediaServiceImpl/
//! WxMaKefuServiceImpl/WxMaAnalysisServiceImpl/WxMaCodeServiceImpl/
//! WxMaExpressServiceImpl/WxMaSecurityServiceImpl/WxMaSettingServiceImpl。
//! 由 Wave 3 统一装配到门面实现。

//! 模块文件位于 `api/impl/` 根目录（`wx_ma_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。

#[path = "wx_ma_analysis_service_impl.rs"]
pub mod wx_ma_analysis_service_impl;
#[path = "wx_ma_code_service_impl.rs"]
pub mod wx_ma_code_service_impl;
#[path = "wx_ma_express_service_impl.rs"]
pub mod wx_ma_express_service_impl;
#[path = "wx_ma_kefu_service_impl.rs"]
pub mod wx_ma_kefu_service_impl;
#[path = "wx_ma_media_service_impl.rs"]
pub mod wx_ma_media_service_impl;
#[path = "wx_ma_msg_service_impl.rs"]
pub mod wx_ma_msg_service_impl;
#[path = "wx_ma_security_service_impl.rs"]
pub mod wx_ma_security_service_impl;
#[path = "wx_ma_setting_service_impl.rs"]
pub mod wx_ma_setting_service_impl;
#[path = "wx_ma_user_service_impl.rs"]
pub mod wx_ma_user_service_impl;

pub use wx_ma_analysis_service_impl::WxMaAnalysisServiceImpl;
pub use wx_ma_code_service_impl::WxMaCodeServiceImpl;
pub use wx_ma_express_service_impl::WxMaExpressServiceImpl;
pub use wx_ma_kefu_service_impl::WxMaKefuServiceImpl;
pub use wx_ma_media_service_impl::WxMaMediaServiceImpl;
pub use wx_ma_msg_service_impl::WxMaMsgServiceImpl;
pub use wx_ma_security_service_impl::WxMaSecurityServiceImpl;
pub use wx_ma_setting_service_impl::WxMaSettingServiceImpl;
pub use wx_ma_user_service_impl::WxMaUserServiceImpl;
