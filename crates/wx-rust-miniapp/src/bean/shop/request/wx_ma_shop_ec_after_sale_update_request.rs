//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopEcAfterSaleUpdateRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopEcAfterSaleUpdateRequest {
    #[serde(rename = "out_aftersale_id", default)]
    pub out_aftersale_id: String,
    #[serde(rename = "aftersale_id", default)]
    pub aftersale_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "orderamt", default)]
    pub orderamt: i32,
    #[serde(rename = "refund_reason", default)]
    pub refund_reason: String,
    #[serde(rename = "refund_reason_type", default)]
    pub refund_reason_type: i32,
    #[serde(rename = "media_list", default)]
    pub media_list: Vec<MediaListBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MediaListBean {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "url", default)]
    pub url: String,
}
