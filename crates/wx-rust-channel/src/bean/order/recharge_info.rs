//! 对应 Java `me.chanjar.weixin.channel.bean.order.RechargeInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RechargeInfo {
    #[serde(rename = "account_no", default)]
    pub account_no: String,
    #[serde(rename = "account_type", default)]
    pub account_type: String,
    #[serde(rename = "wx_openid", default)]
    pub wx_open_id: String,
}
