//! 内容服务组（G2）子服务 trait。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api` 包中内容类子服务接口
//! （subscribe/share/scheme/link/qrcode/jsapi/plugin/run/openapi/internet）。
//!
//! 模块文件位于 `api/` 根目录（`wx_ma_<域>_service.rs`，与任务文件布局
//! 一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式
//! 指回根目录文件。

#[path = "wx_ma_internet_service.rs"]
pub mod wx_ma_internet_service;
#[path = "wx_ma_jsapi_service.rs"]
pub mod wx_ma_jsapi_service;
#[path = "wx_ma_link_service.rs"]
pub mod wx_ma_link_service;
#[path = "wx_ma_open_api_service.rs"]
pub mod wx_ma_open_api_service;
#[path = "wx_ma_plugin_service.rs"]
pub mod wx_ma_plugin_service;
#[path = "wx_ma_qrcode_service.rs"]
pub mod wx_ma_qrcode_service;
#[path = "wx_ma_run_service.rs"]
pub mod wx_ma_run_service;
#[path = "wx_ma_scheme_service.rs"]
pub mod wx_ma_scheme_service;
#[path = "wx_ma_share_service.rs"]
pub mod wx_ma_share_service;
#[path = "wx_ma_subscribe_service.rs"]
pub mod wx_ma_subscribe_service;

pub use wx_ma_internet_service::WxMaInternetService;
pub use wx_ma_jsapi_service::WxMaJsapiService;
pub use wx_ma_link_service::WxMaLinkService;
pub use wx_ma_open_api_service::WxMaOpenApiService;
pub use wx_ma_plugin_service::WxMaPluginService;
pub use wx_ma_qrcode_service::WxMaQrcodeService;
pub use wx_ma_run_service::WxMaRunService;
pub use wx_ma_scheme_service::WxMaSchemeService;
pub use wx_ma_share_service::WxMaShareService;
pub use wx_ma_subscribe_service::WxMaSubscribeService;
