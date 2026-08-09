//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleAcceptExchangeReshipParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleAcceptExchangeReshipParam {
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
}
