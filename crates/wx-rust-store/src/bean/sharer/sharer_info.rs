//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.SharerInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SharerInfo；对应 Java com.binarywang.wxjava.store.bean.sharer.SharerInfo.java。
pub struct SharerInfo {
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(rename = "unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    #[serde(rename = "nickname", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(rename = "bind_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_time: Option<i64>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
}
