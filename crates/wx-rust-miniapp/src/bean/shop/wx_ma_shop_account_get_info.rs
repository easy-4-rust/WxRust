//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopAccountGetInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAccountGetInfo {
    #[serde(rename = "brand_id", default)]
    pub brand_id: i64,
    #[serde(rename = "brand_wording", default)]
    pub brand_wording: String,
}
