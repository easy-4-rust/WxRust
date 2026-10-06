//! 对应 Java `com.binarywang.wxjava.store.bean.product.AfterSaleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AfterSaleInfo；对应 Java com.binarywang.wxjava.store.bean.product.AfterSaleInfo.java。
pub struct AfterSaleInfo {
    #[serde(rename = "after_sale_address_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_address_id: Option<i64>,
}
