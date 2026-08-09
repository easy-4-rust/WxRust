//! 对应 Java `me.chanjar.weixin.open.bean.minishop.goods.WxMinishopAddGoodsSpuData.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopAddGoodsSpuData {
    #[serde(rename = "productId", default)]
    pub product_id: i64,
    #[serde(rename = "outProductId", default)]
    pub out_product_id: String,
    #[serde(rename = "createTime", default)]
    pub create_time: String,
}
