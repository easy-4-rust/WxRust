//! 对应 Java `me.chanjar.weixin.channel.bean.order.PrivateNumberSendVerifyCodeParam.java`。

#[allow(unused_imports)]
use super::*;

/// 获取短信验证码请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberSendVerifyCodeParam {
    /// 手机号。
    #[serde(rename = "phone", default)]
    pub phone: String,
}
