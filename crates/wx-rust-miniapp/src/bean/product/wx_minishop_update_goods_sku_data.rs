//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopUpdateGoodsSkuData.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopUpdateGoodsSkuData {
    #[serde(rename = "sku_id", default)]
    pub sku_id: i64,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
}
