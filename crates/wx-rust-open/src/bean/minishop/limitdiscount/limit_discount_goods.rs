//! 对应 Java `me.chanjar.weixin.open.bean.minishop.limitdiscount.LimitDiscountGoods.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::minishop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitDiscountGoods {
    #[serde(rename = "taskId", default)]
    pub task_id: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "productId", default)]
    pub product_id: i64,
    #[serde(rename = "startTime", default)]
    pub start_time: String,
    #[serde(rename = "endTime", default)]
    pub end_time: String,
    #[serde(rename = "limitDiscountSkuList", default)]
    pub limit_discount_sku_list: Vec<LimitDiscountSku>,
}
