//! 对应 Java `cn.binarywang.wx.miniapp.bean.product.WxMinishopOrderResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMinishopOrderResult {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "update_time", default)]
    pub update_time: String,
    #[serde(rename = "order_detail", default)]
    pub order_detail: WxMinishopOrderDetail,
    #[serde(rename = "aftersale_detail", default)]
    pub after_sale_detail: WxMiniOrderAfterSaleDetail,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "ext_info", default)]
    pub ext_info: ExtInfo,
    #[serde(rename = "order_type", default)]
    pub order_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExtInfo {
    #[serde(rename = "customer_notes", default)]
    pub customer_notes: String,
    #[serde(rename = "merchant_notes", default)]
    pub merchant_notes: String,
}
