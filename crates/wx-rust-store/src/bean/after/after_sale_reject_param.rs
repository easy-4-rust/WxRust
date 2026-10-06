//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleRejectParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleRejectParam；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleRejectParam.java。
pub struct AfterSaleRejectParam {
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "reject_reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason: Option<String>,
    #[serde(rename = "reject_reason_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_reason_type: Option<i32>,
    #[serde(rename = "reject_certificates", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reject_certificates: Option<Vec<String>>,
}
