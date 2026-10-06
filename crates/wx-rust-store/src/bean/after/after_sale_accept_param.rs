//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleAcceptParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleAcceptParam；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleAcceptParam.java。
pub struct AfterSaleAcceptParam {
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "address_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_id: Option<String>,
    #[serde(rename = "accept_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_type: Option<i32>,
}
