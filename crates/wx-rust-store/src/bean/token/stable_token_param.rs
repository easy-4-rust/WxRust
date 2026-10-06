//! 对应 Java `com.binarywang.wxjava.store.bean.token.StableTokenParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StableTokenParam；对应 Java com.binarywang.wxjava.store.bean.token.StableTokenParam.java。
pub struct StableTokenParam {
    #[serde(rename = "grant_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<String>,
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    #[serde(rename = "secret", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(rename = "force_refresh", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_refresh: Option<bool>,
}
