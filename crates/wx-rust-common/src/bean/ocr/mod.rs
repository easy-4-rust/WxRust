//! OCR 结果数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.ocr` 包。

pub mod wx_ocr_bank_card_result;
pub mod wx_ocr_biz_license_result;
pub mod wx_ocr_comm_result;
pub mod wx_ocr_driving_license_result;
pub mod wx_ocr_driving_result;
pub mod wx_ocr_id_card_result;
pub mod wx_ocr_img_size;
pub mod wx_ocr_pos;

pub use wx_ocr_bank_card_result::WxOcrBankCardResult;
pub use wx_ocr_biz_license_result::WxOcrBizLicenseResult;
pub use wx_ocr_comm_result::{Items, WxOcrCommResult};
pub use wx_ocr_driving_license_result::WxOcrDrivingLicenseResult;
pub use wx_ocr_driving_result::WxOcrDrivingResult;
pub use wx_ocr_id_card_result::WxOcrIdCardResult;
pub use wx_ocr_img_size::WxOcrImgSize;
pub use wx_ocr_pos::Coordinate;
pub use wx_ocr_pos::WxOcrPos;
