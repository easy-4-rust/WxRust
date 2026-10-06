//! 对应 Java `com.binarywang.wxjava.store.bean.order.PrivateNumberSendVerifyCodeParam.java`。

/// 获取短信验证码请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberSendVerifyCodeParam {
    /// 手机号。
    #[serde(rename = "phone", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
