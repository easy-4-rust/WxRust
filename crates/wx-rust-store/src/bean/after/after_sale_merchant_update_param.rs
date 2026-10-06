//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleMerchantUpdateParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleMerchantUpdateParam；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleMerchantUpdateParam.java。
pub struct AfterSaleMerchantUpdateParam {
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "amount", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    #[serde(rename = "merchant_update_desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_update_desc: Option<String>,
    #[serde(rename = "update_reason_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_reason_type: Option<i32>,
    #[serde(rename = "merchant_update_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_update_type: Option<i32>,
    #[serde(rename = "media_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_ids: Option<Vec<String>>,
}
