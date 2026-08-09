//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopSkuAttribute.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSkuAttribute {
    #[serde(rename = "attr_key", default)]
    pub attr_key: String,
    #[serde(rename = "attr_value", default)]
    pub attr_value: String,
}
