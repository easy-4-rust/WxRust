//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleAcceptParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleAcceptParam {
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: String,
    #[serde(rename = "address_id", default)]
    pub address_id: String,
    #[serde(rename = "accept_type", default)]
    pub accept_type: i32,
}
