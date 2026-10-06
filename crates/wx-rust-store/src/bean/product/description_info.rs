//! 对应 Java `com.binarywang.wxjava.store.bean.product.DescriptionInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DescriptionInfo；对应 Java com.binarywang.wxjava.store.bean.product.DescriptionInfo.java。
pub struct DescriptionInfo {
    #[serde(rename = "desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(rename = "imgs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imgs: Option<Vec<String>>,
}
