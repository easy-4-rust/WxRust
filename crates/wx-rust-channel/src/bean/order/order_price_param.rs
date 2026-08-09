//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderPriceParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderPriceParam {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "change_express", default)]
    pub change_express: bool,
    #[serde(rename = "express_fee", default)]
    pub express_fee: i32,
    #[serde(rename = "change_order_infos", default)]
    pub change_order_infos: Vec<ChangeOrderInfo>,
}
