//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleIdParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleIdParam；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleIdParam.java。
pub struct AfterSaleIdParam {
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
}
