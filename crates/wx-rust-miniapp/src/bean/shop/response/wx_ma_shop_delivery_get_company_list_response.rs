//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopDeliveryGetCompanyListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopDeliveryGetCompanyListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "company_list", default)]
    pub company_list: Vec<CompanyListBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompanyListBean {
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}
