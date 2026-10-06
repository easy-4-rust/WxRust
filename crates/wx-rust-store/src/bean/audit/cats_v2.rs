//! 对应 Java `com.binarywang.wxjava.store.bean.audit.CatsV2.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CatsV2；对应 Java com.binarywang.wxjava.store.bean.audit.CatsV2.java。
pub struct CatsV2 {
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat_id: Option<String>,
}
