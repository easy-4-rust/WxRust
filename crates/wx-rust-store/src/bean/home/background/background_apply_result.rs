//! 对应 Java `com.binarywang.wxjava.store.bean.home.background.BackgroundApplyResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BackgroundApplyResult；对应 Java com.binarywang.wxjava.store.bean.home.background.BackgroundApplyResult.java。
pub struct BackgroundApplyResult {
    #[serde(rename = "apply_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_id: Option<i32>,
    #[serde(rename = "state", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    #[serde(rename = "audit_desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_desc: Option<String>,
    #[serde(rename = "img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
}
