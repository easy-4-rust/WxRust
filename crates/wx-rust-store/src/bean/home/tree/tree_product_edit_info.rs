//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeProductEditInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TreeProductEditInfo；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeProductEditInfo.java。
pub struct TreeProductEditInfo {
    #[serde(rename = "level_1_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level1_id: Option<i32>,
    #[serde(rename = "level_2_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level2_id: Option<i32>,
    #[serde(rename = "product_ids", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_ids: Option<Vec<i64>>,
}
