//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderOverallData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderOverallData {
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
    #[serde(rename = "pay_sales_finder_cnt", default)]
    pub pay_sales_finder_cnt: String,
    #[serde(rename = "pay_product_id_cnt", default)]
    pub pay_product_id_cnt: String,
    #[serde(rename = "click_to_pay_uv_ratio", default)]
    pub click_to_pay_uv_ratio: f64,
}
