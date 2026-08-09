//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopAfterSaleUploadReturnInfoRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAfterSaleUploadReturnInfoRequest {
    #[serde(rename = "out_aftersale_id", default)]
    pub out_aftersale_id: String,
    #[serde(rename = "aftersale_id", default)]
    pub aftersale_id: i64,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "waybill_id", default)]
    pub waybill_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}
