//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeOrderListResponse.java`。

/// 保障单列表响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderListResponse {
    /// 错误码。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 保障单总数。
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
    /// 保障单列表。
    #[serde(rename = "guarantee_order_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_list: Option<Vec<GuaranteeOrderListItem>>,
}

/// 保障单列表项。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderListItem {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
    /// 保障单状态。
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 商品信息列表。
    #[serde(rename = "product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<Vec<GuaranteeListItemProductInfo>>,
}

/// 列表商品信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeListItemProductInfo {
    /// 商品 SPU ID。
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
