//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaLiveResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaLiveResult {
    #[serde(rename = "total", default)]
    pub total: i32,
    #[serde(rename = "auditId", default)]
    pub audit_id: i64,
    #[serde(rename = "goodsId", default)]
    pub goods_id: i32,
    #[serde(rename = "goods", default)]
    pub goods: Vec<Goods>,
    #[serde(rename = "room_info", default)]
    pub room_infos: Vec<RoomInfo>,
    #[serde(rename = "live_replay", default)]
    pub live_replay: Vec<LiveReplay>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Goods {
    #[serde(rename = "goods_id", default)]
    pub goods_id: i32,
    #[serde(rename = "cover_img_url", default)]
    pub cover_img_url: String,
    #[serde(rename = "cover_img", default)]
    pub cover_img: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "price_type", default)]
    pub price_type: i32,
    #[serde(rename = "audit_status", default)]
    pub audit_status: i32,
    #[serde(
        rename = "price",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub price: String,
    #[serde(
        rename = "price2",
        default,
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub price2: String,
    #[serde(rename = "third_party_tag", default)]
    pub third_party_tag: String,
    #[serde(rename = "thirdPartyAppid", default)]
    pub third_party_appid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RoomInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "roomid", default)]
    pub room_id: i32,
    #[serde(rename = "cover_img", default)]
    pub cover_img: String,
    #[serde(rename = "share_img", default)]
    pub share_img: String,
    #[serde(rename = "live_status", default)]
    pub live_status: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "anchor_name", default)]
    pub anchor_name: String,
    #[serde(rename = "anchor_wechat", default)]
    pub anchor_wechat: String,
    #[serde(rename = "anchor_img", default)]
    pub anchor_img: String,
    #[serde(rename = "live_type", default)]
    pub r#type: i32,
    #[serde(rename = "screen_type", default)]
    pub screen_type: i32,
    #[serde(rename = "close_like", default)]
    pub close_like: i32,
    #[serde(rename = "close_goods", default)]
    pub close_goods: i32,
    #[serde(rename = "close_comment", default)]
    pub close_comment: i32,
    #[serde(rename = "close_kf", default)]
    pub close_kf: i32,
    #[serde(rename = "close_replay", default)]
    pub close_replay: i32,
    #[serde(rename = "is_feeds_public", default)]
    pub is_feeds_public: i32,
    #[serde(rename = "creater_openid", default)]
    pub creater_openid: String,
    #[serde(rename = "feeds_img", default)]
    pub feeds_img: String,
    #[serde(rename = "goods", default)]
    pub goods: Vec<Goods>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveReplay {
    #[serde(rename = "expire_time", default)]
    pub expire_time: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "media_url", default)]
    pub media_url: String,
}

impl WxMaLiveResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaLiveResult 解析失败: {e}"))
    }
}
