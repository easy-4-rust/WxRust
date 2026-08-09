//! 对应 Java `cn.binarywang.wx.miniapp.bean.live.WxMaLiveGoodInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaLiveGoodInfo {
    #[serde(rename = "goodsId", default)]
    pub goods_id: i32,
    #[serde(rename = "coverImgUrl", default)]
    pub cover_img_url: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "priceType", default)]
    pub price_type: i32,
    #[serde(
        rename = "price",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal",
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub price: String,
    #[serde(
        rename = "price2",
        default,
        serialize_with = "crate::bean::serde_util::ser_decimal",
        deserialize_with = "crate::bean::serde_util::de_num_or_str"
    )]
    pub price2: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "thirdPartyTag", default)]
    pub third_party_tag: String,
    #[serde(rename = "goodsKey", default)]
    pub goods_key: Vec<String>,
    #[serde(rename = "thirdPartyAppid", default)]
    pub third_party_appid: String,
}
