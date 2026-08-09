//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopSpuWithoutAuditInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSpuWithoutAuditInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "skus", default)]
    pub skus: Vec<WxMaShopSkuWithoutAuditInfo>,
}
