//! 对应 Java `com.binarywang.wxjava.store.bean.order.PrivateNumberGetPhoneResponse.java`。

use super::PrivateNumberPhoneInfo;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_list: Option<Vec<PrivateNumberPhoneInfo>>,
}
