//! 对应 Java `com.binarywang.wxjava.store.bean.home.tree.TreeAuditResultDetail.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TreeAuditResultDetail；对应 Java com.binarywang.wxjava.store.bean.home.tree.TreeAuditResultDetail.java。
pub struct TreeAuditResultDetail {
    #[serde(rename = "level_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_id: Option<i32>,
    #[serde(rename = "result_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<i32>,
}
