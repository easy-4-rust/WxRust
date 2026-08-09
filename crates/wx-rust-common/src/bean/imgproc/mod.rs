//! 图片处理结果数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.imgproc` 包。

pub mod wx_img_proc_ai_crop_result;
pub mod wx_img_proc_qr_code_result;
pub mod wx_img_proc_super_resolution_result;

pub use wx_img_proc_ai_crop_result::{
    ImgSize as AiCropImgSize, Results as AiCropResults, WxImgProcAiCropResult,
};
pub use wx_img_proc_qr_code_result::{
    CodeResults as QrCodeResults, ImgSize as QrImgSize, WxImgProcQrCodeResult,
};
pub use wx_img_proc_super_resolution_result::WxImgProcSuperResolutionResult;
