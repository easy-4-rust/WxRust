//! 能力服务组（G4）子服务实现注册。
//!
//! 本组实现 Java `cn.binarywang.wx.miniapp.api.impl` 中 16 个能力类子服务：
//! WxMaLiveServiceImpl/WxMaLiveGoodsServiceImpl/WxMaLiveMemberServiceImpl/
//! WxMaCloudServiceImpl/WxMaVodServiceImpl/WxMaXPayServiceImpl/
//! WxMaMarketingServiceImpl/WxMaPromotionServiceImpl/WxMaIntracityServiceImpl/
//! WxMaComplaintServiceImpl/WxMaDeviceSubscribeServiceImpl/WxMaFaceServiceImpl/
//! WxMaReimburseInvoiceServiceImpl/WxMaQrcodeJumpServiceImpl/
//! WxMaOcrServiceImpl/WxMaImgProcServiceImpl（后两者无独立接口，直接实现
//! common 的 `WxOcrService`/`WxImgProcService` trait）。
//! 由 Wave 3 统一装配到门面实现。
//!
//! 模块文件位于 `api/impl/` 根目录（`wx_ma_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。

#[path = "wx_ma_cloud_service_impl.rs"]
pub mod wx_ma_cloud_service_impl;
#[path = "wx_ma_complaint_service_impl.rs"]
pub mod wx_ma_complaint_service_impl;
#[path = "wx_ma_device_subscribe_service_impl.rs"]
pub mod wx_ma_device_subscribe_service_impl;
#[path = "wx_ma_face_service_impl.rs"]
pub mod wx_ma_face_service_impl;
#[path = "wx_ma_img_proc_service_impl.rs"]
pub mod wx_ma_img_proc_service_impl;
#[path = "wx_ma_intracity_service_impl.rs"]
pub mod wx_ma_intracity_service_impl;
#[path = "wx_ma_live_goods_service_impl.rs"]
pub mod wx_ma_live_goods_service_impl;
#[path = "wx_ma_live_member_service_impl.rs"]
pub mod wx_ma_live_member_service_impl;
#[path = "wx_ma_live_service_impl.rs"]
pub mod wx_ma_live_service_impl;
#[path = "wx_ma_marketing_service_impl.rs"]
pub mod wx_ma_marketing_service_impl;
#[path = "wx_ma_ocr_service_impl.rs"]
pub mod wx_ma_ocr_service_impl;
#[path = "wx_ma_promotion_service_impl.rs"]
pub mod wx_ma_promotion_service_impl;
#[path = "wx_ma_qrcode_jump_service_impl.rs"]
pub mod wx_ma_qrcode_jump_service_impl;
#[path = "wx_ma_reimburse_invoice_service_impl.rs"]
pub mod wx_ma_reimburse_invoice_service_impl;
#[path = "wx_ma_vod_service_impl.rs"]
pub mod wx_ma_vod_service_impl;
#[path = "wx_ma_xpay_service_impl.rs"]
pub mod wx_ma_xpay_service_impl;

pub use wx_ma_cloud_service_impl::WxMaCloudServiceImpl;
pub use wx_ma_complaint_service_impl::WxMaComplaintServiceImpl;
pub use wx_ma_device_subscribe_service_impl::WxMaDeviceSubscribeServiceImpl;
pub use wx_ma_face_service_impl::WxMaFaceServiceImpl;
pub use wx_ma_img_proc_service_impl::WxMaImgProcServiceImpl;
pub use wx_ma_intracity_service_impl::WxMaIntracityServiceImpl;
pub use wx_ma_live_goods_service_impl::WxMaLiveGoodsServiceImpl;
pub use wx_ma_live_member_service_impl::WxMaLiveMemberServiceImpl;
pub use wx_ma_live_service_impl::WxMaLiveServiceImpl;
pub use wx_ma_marketing_service_impl::WxMaMarketingServiceImpl;
pub use wx_ma_ocr_service_impl::WxMaOcrServiceImpl;
pub use wx_ma_promotion_service_impl::WxMaPromotionServiceImpl;
pub use wx_ma_qrcode_jump_service_impl::WxMaQrcodeJumpServiceImpl;
pub use wx_ma_reimburse_invoice_service_impl::WxMaReimburseInvoiceServiceImpl;
pub use wx_ma_vod_service_impl::WxMaVodServiceImpl;
pub use wx_ma_xpay_service_impl::WxMaXPayServiceImpl;
