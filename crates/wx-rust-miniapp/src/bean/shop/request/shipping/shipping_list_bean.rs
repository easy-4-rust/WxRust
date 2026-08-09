//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.request.shipping.ShippingListBean.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::request::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShippingListBean {
    #[serde(rename = "tracking_no", default)]
    pub tracking_no: String,
    #[serde(rename = "express_company", default)]
    pub express_company: String,
    #[serde(rename = "item_desc", default)]
    pub item_desc: String,
    #[serde(rename = "contact", default)]
    pub contact: ContactBean,
}
