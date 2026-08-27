//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductSchemeResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 获取商品移动应用跳转 scheme 码响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductSchemeResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// scheme 链接。
    #[serde(rename = "openlink", default)]
    pub openlink: String,
}
