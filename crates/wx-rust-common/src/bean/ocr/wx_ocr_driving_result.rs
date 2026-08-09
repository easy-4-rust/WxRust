//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrDrivingResult`（由 gen_bean_structs.py 生成）。

use super::wx_ocr_img_size::WxOcrImgSize;
use super::wx_ocr_pos::WxOcrPos;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrDrivingResult {
    /// plateNum
    #[serde(rename = "plate_num", default)]
    pub plate_num: String,
    /// vehicleType
    #[serde(rename = "vehicle_type", default)]
    pub vehicle_type: String,
    /// owner
    #[serde(rename = "owner", default)]
    pub owner: String,
    /// addr
    #[serde(rename = "addr", default)]
    pub addr: String,
    /// useCharacter
    #[serde(rename = "use_character", default)]
    pub use_character: String,
    /// model
    #[serde(rename = "model", default)]
    pub model: String,
    /// vin
    #[serde(rename = "vin", default)]
    pub vin: String,
    /// engineNum
    #[serde(rename = "engine_num", default)]
    pub engine_num: String,
    /// registerDate
    #[serde(rename = "register_date", default)]
    pub register_date: String,
    /// issueDate
    #[serde(rename = "issue_date", default)]
    pub issue_date: String,
    /// plateNumB
    #[serde(rename = "plate_num_b", default)]
    pub plate_num_b: String,
    /// record
    #[serde(rename = "record", default)]
    pub record: String,
    /// passengersNum
    #[serde(rename = "passengers_num", default)]
    pub passengers_num: String,
    /// totalQuality
    #[serde(rename = "total_quality", default)]
    pub total_quality: String,
    /// prepareQuality
    #[serde(rename = "prepare_quality", default)]
    pub prepare_quality: String,
    /// overallSize
    #[serde(rename = "overall_size", default)]
    pub overall_size: String,
    /// cardPositionFront
    #[serde(rename = "card_position_front", default)]
    pub card_position_front: CardPosition,
    /// cardPositionBack
    #[serde(rename = "card_position_back", default)]
    pub card_position_back: CardPosition,
    /// imgSize
    #[serde(rename = "img_size", default)]
    pub img_size: WxOcrImgSize,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardPosition {
    /// pos
    #[serde(rename = "pos", default)]
    pub pos: WxOcrPos,
}
