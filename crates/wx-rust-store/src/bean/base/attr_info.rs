//! 对应 Java `com.binarywang.wxjava.store.bean.base.AttrInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AttrInfo；对应 Java com.binarywang.wxjava.store.bean.base.AttrInfo.java。
pub struct AttrInfo {
    #[serde(rename = "attr_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "attr_value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
