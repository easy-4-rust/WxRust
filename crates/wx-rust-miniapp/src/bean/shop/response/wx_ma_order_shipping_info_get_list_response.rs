//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaOrderShippingInfoGetListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOrderShippingInfoGetListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "last_index", default)]
    pub last_index: String,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
    #[serde(rename = "order_list", default)]
    pub order_list: Vec<Order>,
}
