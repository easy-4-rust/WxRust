//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeOrderListResponse.java`。

#[allow(unused_imports)]
use super::*;

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
    pub total_num: i32,
    /// 保障单列表。
    #[serde(rename = "guarantee_order_list", default)]
    pub guarantee_order_list: Vec<GuaranteeOrderListItem>,
}

/// 保障单列表项。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderListItem {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
    /// 保障单状态。
    #[serde(rename = "status", default)]
    pub status: String,
    /// 商品信息列表。
    #[serde(rename = "product_info", default)]
    pub product_info: Vec<GuaranteeListItemProductInfo>,
}

/// 列表商品信息。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeListItemProductInfo {
    /// 商品 SPU ID。
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
