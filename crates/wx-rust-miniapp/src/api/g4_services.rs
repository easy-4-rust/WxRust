//! 能力服务组（G4）子服务 trait 注册。
//!
//! 本组实现 Java `cn.binarywang.wx.miniapp.api` 中 14 个能力类子服务接口：
//! WxMaLiveService/WxMaLiveGoodsService/WxMaLiveMemberService/
//! WxMaCloudService/WxMaVodService/WxMaXPayService/WxMaMarketingService/
//! WxMaPromotionService/WxMaIntracityService/WxMaComplaintService/
//! WxMaDeviceSubscribeService/WxMaFaceService/WxMaReimburseInvoiceService/
//! WxMaQrcodeJumpService。由 Wave 3 统一装配到门面。
//!
//! 模块文件位于 `api/` 根目录（`wx_ma_<域>_service.rs`，与任务文件布局
//! 一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]` 显式
//! 指回根目录文件。

#[path = "wx_ma_cloud_service.rs"]
pub mod wx_ma_cloud_service;
#[path = "wx_ma_complaint_service.rs"]
pub mod wx_ma_complaint_service;
#[path = "wx_ma_device_subscribe_service.rs"]
pub mod wx_ma_device_subscribe_service;
#[path = "wx_ma_face_service.rs"]
pub mod wx_ma_face_service;
#[path = "wx_ma_intracity_service.rs"]
pub mod wx_ma_intracity_service;
#[path = "wx_ma_live_goods_service.rs"]
pub mod wx_ma_live_goods_service;
#[path = "wx_ma_live_member_service.rs"]
pub mod wx_ma_live_member_service;
#[path = "wx_ma_live_service.rs"]
pub mod wx_ma_live_service;
#[path = "wx_ma_marketing_service.rs"]
pub mod wx_ma_marketing_service;
#[path = "wx_ma_promotion_service.rs"]
pub mod wx_ma_promotion_service;
#[path = "wx_ma_qrcode_jump_service.rs"]
pub mod wx_ma_qrcode_jump_service;
#[path = "wx_ma_reimburse_invoice_service.rs"]
pub mod wx_ma_reimburse_invoice_service;
#[path = "wx_ma_vod_service.rs"]
pub mod wx_ma_vod_service;
#[path = "wx_ma_xpay_service.rs"]
pub mod wx_ma_xpay_service;

pub use wx_ma_cloud_service::WxMaCloudService;
pub use wx_ma_complaint_service::WxMaComplaintService;
pub use wx_ma_device_subscribe_service::WxMaDeviceSubscribeService;
pub use wx_ma_face_service::WxMaFaceService;
pub use wx_ma_intracity_service::WxMaIntracityService;
pub use wx_ma_live_goods_service::WxMaLiveGoodsService;
pub use wx_ma_live_member_service::WxMaLiveMemberService;
pub use wx_ma_live_service::WxMaLiveService;
pub use wx_ma_marketing_service::WxMaMarketingService;
pub use wx_ma_promotion_service::WxMaPromotionService;
pub use wx_ma_qrcode_jump_service::WxMaQrcodeJumpService;
pub use wx_ma_reimburse_invoice_service::WxMaReimburseInvoiceService;
pub use wx_ma_vod_service::WxMaVodService;
pub use wx_ma_xpay_service::WxMaXPayService;
