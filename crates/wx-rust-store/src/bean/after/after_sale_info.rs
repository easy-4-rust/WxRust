//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{
    AfterSaleDetail, AfterSaleExchangeDeliveryInfo, AfterSaleExchangeProductInfo,
    AfterSaleProductInfo, AfterSaleVirtualNumberInfo, MerchantUploadInfo, RefundInfo, RefundResp,
    ReturnInfo,
};

/// 微信小店 AfterSaleInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleInfo {
    #[serde(rename = "after_sale_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_order_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openid: Option<String>,
    #[serde(rename = "unionid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unionid: Option<String>,
    #[serde(rename = "product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<AfterSaleProductInfo>,
    #[serde(rename = "details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<AfterSaleDetail>,
    #[serde(rename = "refund_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_info: Option<RefundInfo>,
    #[serde(rename = "return_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_info: Option<ReturnInfo>,
    #[serde(rename = "merchant_upload_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_upload_info: Option<MerchantUploadInfo>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    #[serde(rename = "update_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    #[serde(rename = "reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "reason_text", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_text: Option<String>,
    #[serde(rename = "refund_resp", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_resp: Option<RefundResp>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "complaint_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complaint_id: Option<String>,
    #[serde(rename = "deadline", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline: Option<i64>,
    #[serde(rename = "exchange_product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_product_info: Option<AfterSaleExchangeProductInfo>,
    #[serde(rename = "exchange_delivery_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_delivery_info: Option<AfterSaleExchangeDeliveryInfo>,
    #[serde(rename = "virtual_tel_num_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_tel_num_info: Option<AfterSaleVirtualNumberInfo>,
}
