//! 对应 Java `me.chanjar.weixin.channel.bean.order.PrivateNumberGetPhoneResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 获取小店手机号认证状态响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberGetPhoneResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 手机号认证信息列表。
    #[serde(rename = "phone_list", default)]
    pub phone_list: Vec<PrivateNumberPhoneInfo>,
}
