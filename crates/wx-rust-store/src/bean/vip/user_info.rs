//! 对应 Java `com.binarywang.wxjava.store.bean.vip.UserInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 UserInfo；对应 Java com.binarywang.wxjava.store.bean.vip.UserInfo.java。
pub struct UserInfo {
    #[serde(rename = "phone_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
}
