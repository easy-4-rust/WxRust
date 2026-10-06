//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeOrderInfoResponse.java`。

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order: Option<GuaranteeOrderDetail>,
}

/// 保障单详情。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderDetail {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
    /// 保障单状态。
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 商品信息。
    #[serde(rename = "product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<GuaranteeProductInfo>,
}

/// 保障单商品信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeProductInfo {
    /// 商品 SPU ID。
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
