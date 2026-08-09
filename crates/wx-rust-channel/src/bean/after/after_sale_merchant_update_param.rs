//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleMerchantUpdateParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleMerchantUpdateParam {
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "amount", default)]
    pub amount: i32,
    #[serde(rename = "merchant_update_desc", default)]
    pub merchant_update_desc: String,
    #[serde(rename = "update_reason_type", default)]
    pub update_reason_type: i32,
    #[serde(rename = "merchant_update_type", default)]
    pub merchant_update_type: i32,
    #[serde(rename = "media_ids", default)]
    pub media_ids: Vec<String>,
}
