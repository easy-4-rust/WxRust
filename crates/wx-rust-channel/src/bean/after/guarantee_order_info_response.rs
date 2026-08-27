//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeOrderInfoResponse.java`。

#[allow(unused_imports)]
use super::*;

/// 保障单详情响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderInfoResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 保障单详情。
    #[serde(rename = "guarantee_order", default)]
    pub guarantee_order: GuaranteeOrderDetail,
}

/// 保障单详情。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderDetail {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
    /// 保障单状态。
    #[serde(rename = "status", default)]
    pub status: String,
    /// 商品信息。
    #[serde(rename = "product_info", default)]
    pub product_info: GuaranteeProductInfo,
}

/// 保障单商品信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeProductInfo {
    /// 商品 SPU ID。
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
