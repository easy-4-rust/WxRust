//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleInfo {
    #[serde(rename = "after_sale_order_id", default)]
    pub after_sale_order_id: String,
    #[serde(rename = "status", default)]
    pub status: String,
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "unionid", default)]
    pub unionid: String,
    #[serde(rename = "product_info", default)]
    pub product_info: AfterSaleProductInfo,
    #[serde(rename = "details", default)]
    pub details: AfterSaleDetail,
    #[serde(rename = "refund_info", default)]
    pub refund_info: RefundInfo,
    #[serde(rename = "return_info", default)]
    pub return_info: ReturnInfo,
    #[serde(rename = "merchant_upload_info", default)]
    pub merchant_upload_info: MerchantUploadInfo,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "reason_text", default)]
    pub reason_text: String,
    #[serde(rename = "refund_resp", default)]
    pub refund_resp: RefundResp,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "complaint_id", default)]
    pub complaint_id: String,
    #[serde(rename = "deadline", default)]
    pub deadline: i64,
    #[serde(rename = "exchange_product_info", default)]
    pub exchange_product_info: AfterSaleExchangeProductInfo,
    #[serde(rename = "exchange_delivery_info", default)]
    pub exchange_delivery_info: AfterSaleExchangeDeliveryInfo,
    #[serde(rename = "virtual_tel_num_info", default)]
    pub virtual_tel_num_info: AfterSaleVirtualNumberInfo,
}
