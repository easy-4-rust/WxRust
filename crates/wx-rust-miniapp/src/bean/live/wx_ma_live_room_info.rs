//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaLiveRoomInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaLiveRoomInfo {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "coverImg", default)]
    pub cover_img: String,
    #[serde(rename = "startTime", default)]
    pub start_time: i64,
    #[serde(rename = "endTime", default)]
    pub end_time: i64,
    #[serde(rename = "anchorName", default)]
    pub anchor_name: String,
    #[serde(rename = "anchorWechat", default)]
    pub anchor_wechat: String,
    #[serde(rename = "subAnchorWechat", default)]
    pub sub_anchor_wechat: String,
    #[serde(rename = "createrWechat", default)]
    pub creater_wechat: String,
    #[serde(rename = "shareImg", default)]
    pub share_img: String,
    #[serde(rename = "feedsImg", default)]
    pub feeds_img: String,
    #[serde(rename = "anchorImg", default)]
    pub anchor_img: String,
    #[serde(rename = "isFeedsPublic", default)]
    pub is_feeds_public: i32,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "screenType", default)]
    pub screen_type: i32,
    #[serde(rename = "closeLike", default)]
    pub close_like: i32,
    #[serde(rename = "closeGoods", default)]
    pub close_goods: i32,
    #[serde(rename = "closeComment", default)]
    pub close_comment: i32,
    #[serde(rename = "closeReplay", default)]
    pub close_replay: i32,
    #[serde(rename = "closeShare", default)]
    pub close_share: i32,
    #[serde(rename = "closeKf", default)]
    pub close_kf: i32,
}
