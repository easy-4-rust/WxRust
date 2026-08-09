//! 对应 Java `me.chanjar.weixin.channel.bean.order.ChangeSkuInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChangeSkuInfo {
    #[serde(rename = "preshipment_change_sku_state", default)]
    pub preshipment_change_sku_state: i32,
    #[serde(rename = "old_sku_id", default)]
    pub old_sku_id: String,
    #[serde(rename = "new_sku_id", default)]
    pub new_sku_id: String,
    #[serde(rename = "ddl_time_stamp", default)]
    pub deadline_time_stamp: i32,
}
