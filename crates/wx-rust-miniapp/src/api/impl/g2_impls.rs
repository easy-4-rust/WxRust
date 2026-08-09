//! 内容服务组（G2）子服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl` 包中内容类子服务实现
//! （subscribe/share/scheme/link/qrcode/jsapi/plugin/run/openapi/internet）。
//!
//! 模块文件位于 `api/impl/` 根目录（`wx_ma_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。

#[path = "wx_ma_internet_service_impl.rs"]
pub mod wx_ma_internet_service_impl;
#[path = "wx_ma_jsapi_service_impl.rs"]
pub mod wx_ma_jsapi_service_impl;
#[path = "wx_ma_link_service_impl.rs"]
pub mod wx_ma_link_service_impl;
#[path = "wx_ma_open_api_service_impl.rs"]
pub mod wx_ma_open_api_service_impl;
#[path = "wx_ma_plugin_service_impl.rs"]
pub mod wx_ma_plugin_service_impl;
#[path = "wx_ma_qrcode_service_impl.rs"]
pub mod wx_ma_qrcode_service_impl;
#[path = "wx_ma_run_service_impl.rs"]
pub mod wx_ma_run_service_impl;
#[path = "wx_ma_scheme_service_impl.rs"]
pub mod wx_ma_scheme_service_impl;
#[path = "wx_ma_share_service_impl.rs"]
pub mod wx_ma_share_service_impl;
#[path = "wx_ma_subscribe_service_impl.rs"]
pub mod wx_ma_subscribe_service_impl;

pub use wx_ma_internet_service_impl::WxMaInternetServiceImpl;
pub use wx_ma_jsapi_service_impl::WxMaJsapiServiceImpl;
pub use wx_ma_link_service_impl::WxMaLinkServiceImpl;
pub use wx_ma_open_api_service_impl::WxMaOpenApiServiceImpl;
pub use wx_ma_plugin_service_impl::WxMaPluginServiceImpl;
pub use wx_ma_qrcode_service_impl::WxMaQrcodeServiceImpl;
pub use wx_ma_run_service_impl::WxMaRunServiceImpl;
pub use wx_ma_scheme_service_impl::WxMaSchemeServiceImpl;
pub use wx_ma_share_service_impl::WxMaShareServiceImpl;
pub use wx_ma_subscribe_service_impl::WxMaSubscribeServiceImpl;
