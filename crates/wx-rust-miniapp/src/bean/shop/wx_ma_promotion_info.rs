//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaPromotionInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionInfo {
    #[serde(rename = "finder_username", default)]
    pub finder_username: String,
    #[serde(rename = "finder_nickname", default)]
    pub finder_nickname: String,
    #[serde(rename = "sharer_openid", default)]
    pub sharer_openid: String,
    #[serde(rename = "live_start_time", default)]
    pub live_start_time: String,
}
