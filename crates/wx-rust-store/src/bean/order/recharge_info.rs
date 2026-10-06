//! 对应 Java `com.binarywang.wxjava.store.bean.order.RechargeInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 RechargeInfo；对应 Java com.binarywang.wxjava.store.bean.order.RechargeInfo.java。
pub struct RechargeInfo {
    #[serde(rename = "account_no", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_no: Option<String>,
    #[serde(rename = "account_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(rename = "wx_openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wx_open_id: Option<String>,
}
