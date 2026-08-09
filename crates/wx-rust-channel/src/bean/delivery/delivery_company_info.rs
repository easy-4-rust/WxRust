//! 对应 Java `me.chanjar.weixin.channel.bean.delivery.DeliveryCompanyInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryCompanyInfo {
    #[serde(rename = "delivery_id", default)]
    pub id: String,
    #[serde(rename = "delivery_name", default)]
    pub name: String,
}
