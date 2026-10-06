//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeProductListResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TreeProductListResult；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeProductListResult.java。
pub struct TreeProductListResult {
    #[serde(rename = "product_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<i64>>,
    #[serde(rename = "total_count", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i32>,
    #[serde(rename = "page_context", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_context: Option<String>,
}
