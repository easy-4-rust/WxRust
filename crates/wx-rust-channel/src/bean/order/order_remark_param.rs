//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderRemarkParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderRemarkParam {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "merchant_notes", default)]
    pub merchant_notes: String,
}
