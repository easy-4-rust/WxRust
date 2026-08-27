//! 对应 Java `me.chanjar.weixin.channel.bean.order.PrivateNumberAddPhoneParam.java`。

#[allow(unused_imports)]
use super::*;

/// 添加待认证手机号请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberAddPhoneParam {
    /// 手机号。
    #[serde(rename = "phone", default)]
    pub phone: String,
}
