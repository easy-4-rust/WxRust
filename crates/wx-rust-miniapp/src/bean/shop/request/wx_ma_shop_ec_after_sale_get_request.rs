//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.WxMaShopEcAfterSaleGetRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopEcAfterSaleGetRequest {
    #[serde(rename = "aftersale_id", default)]
    pub aftersale_id: i64,
    #[serde(rename = "out_aftersale_id", default)]
    pub out_aftersale_id: String,
}
