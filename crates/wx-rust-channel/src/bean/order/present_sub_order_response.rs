//! 对应 Java `me.chanjar.weixin.channel.bean.order.PresentSubOrderResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 获取礼物单的子单列表响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PresentSubOrderResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 子单列表。
    #[serde(rename = "sub_order_ids", default)]
    pub sub_order_ids: Vec<String>,
}
