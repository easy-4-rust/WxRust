//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAfterSaleAddRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAfterSaleAddRequest {
    #[serde(rename = "order_id", default)]
    pub order_id: i64,
    #[serde(rename = "out_order_id", default)]
    pub out_order_id: String,
    #[serde(rename = "out_aftersale_id", default)]
    pub out_aftersale_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "product_info", default)]
    pub product_info: ProductInfosBean,
    #[serde(rename = "orderamt", default)]
    pub orderamt: i64,
    #[serde(rename = "refund_reason", default)]
    pub refund_reason: String,
    #[serde(rename = "refund_reason_type", default)]
    pub refund_reason_type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "finish_all_aftersale", default)]
    pub finish_all_aftersale: i32,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "refund", default)]
    pub refund: i64,
    #[serde(rename = "media_list", default)]
    pub media_list: UploadMediaList,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfosBean {
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "out_sku_id", default)]
    pub out_sku_id: String,
    #[serde(rename = "product_cnt", default)]
    pub product_cnt: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UploadMediaList {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "thumb_url", default)]
    pub thumb_url: String,
}
