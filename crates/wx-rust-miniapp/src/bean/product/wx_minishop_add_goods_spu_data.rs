//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopAddGoodsSpuData.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopAddGoodsSpuData {
    #[serde(rename = "productId", default)]
    pub product_id: i64,
    #[serde(rename = "outProductId", default)]
    pub out_product_id: String,
    #[serde(rename = "createTime", default)]
    pub create_time: String,
    #[serde(rename = "updateTime", default)]
    pub update_time: String,
}
