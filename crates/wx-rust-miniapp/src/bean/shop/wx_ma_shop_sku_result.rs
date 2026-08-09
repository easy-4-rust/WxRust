//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.WxMaShopSkuResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopSkuResult {
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
}
