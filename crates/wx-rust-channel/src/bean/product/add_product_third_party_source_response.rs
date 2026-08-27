//! 对应 Java `me.chanjar.weixin.channel.bean.product.AddProductThirdPartySourceResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 新增第三方货源信息响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddProductThirdPartySourceResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 第三方货源 ID。
    #[serde(rename = "third_party_source_id", default)]
    pub third_party_source_id: i64,
}
