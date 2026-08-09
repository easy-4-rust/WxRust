//! 对应 Java `me.chanjar.weixin.channel.bean.order.OrderAddressParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderAddressParam {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "user_address", default)]
    pub user_address: AddressInfo,
}
