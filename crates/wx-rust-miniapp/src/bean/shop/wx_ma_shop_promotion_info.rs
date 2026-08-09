//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopPromotionInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopPromotionInfo {
    #[serde(rename = "promoter_id", default)]
    pub promoter_id: String,
    #[serde(rename = "finder_nickname", default)]
    pub finder_nickname: String,
    #[serde(rename = "promoter_openid", default)]
    pub promoter_openid: String,
    #[serde(rename = "sharer_openid", default)]
    pub sharer_openid: String,
}
