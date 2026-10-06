//! 对应 Java `com.binarywang.wxjava.store.bean.order.PrivateNumberAddPhoneParam.java`。

/// 添加待认证手机号请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrivateNumberAddPhoneParam {
    /// 手机号。
    #[serde(rename = "phone", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
