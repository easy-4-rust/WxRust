//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderGmvData.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderGmvData {
    #[serde(rename = "pay_gmv", default)]
    pub pay_gmv: String,
    #[serde(rename = "pay_product_id_cnt", default)]
    pub pay_product_id_cnt: String,
    #[serde(rename = "pay_uv", default)]
    pub pay_uv: String,
    #[serde(rename = "refund_gmv", default)]
    pub refund_gmv: String,
    #[serde(rename = "pay_refund_gmv", default)]
    pub pay_refund_gmv: String,
}
