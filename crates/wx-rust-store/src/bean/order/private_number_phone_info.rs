//! 对应 Java `com.binarywang.wxjava.store.bean.order.PrivateNumberPhoneInfo.java`。

/// 手机号认证信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberPhoneInfo {
    /// 手机号。
    #[serde(rename = "phone", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 认证状态：1-待认证，2-认证成功，3-认证失败。
    #[serde(rename = "auth_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<i32>,
}
