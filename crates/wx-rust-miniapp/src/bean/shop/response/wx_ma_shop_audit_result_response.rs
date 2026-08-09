//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopAuditResultResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopAuditResultResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "data", default)]
    pub data: DataBean,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DataBean {
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "brand_id", default)]
    pub brand_id: i32,
    #[serde(rename = "reject_reason", default)]
    pub reject_reason: String,
}
