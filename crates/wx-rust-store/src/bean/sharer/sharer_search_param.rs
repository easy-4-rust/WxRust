//! 对应 Java `com.binarywang.wxjava.store.bean.sharer.SharerSearchParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SharerSearchParam；对应 Java com.binarywang.wxjava.store.bean.sharer.SharerSearchParam.java。
pub struct SharerSearchParam {
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(rename = "username", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
