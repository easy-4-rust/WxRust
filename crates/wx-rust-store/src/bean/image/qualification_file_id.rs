//! 对应 Java `com.binarywang.wxjava.store.bean.image.QualificationFileId.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 QualificationFileId；对应 Java com.binarywang.wxjava.store.bean.image.QualificationFileId.java。
pub struct QualificationFileId {
    #[serde(rename = "file_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
