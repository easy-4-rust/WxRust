//! 对应 Java `me.chanjar.weixin.channel.bean.order.PrivateNumberPhoneInfo.java`。

#[allow(unused_imports)]
use super::*;

/// 手机号认证信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberPhoneInfo {
    /// 手机号。
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// 认证状态：1-待认证，2-认证成功，3-认证失败。
    #[serde(rename = "auth_status", default)]
    pub auth_status: i32,
}
