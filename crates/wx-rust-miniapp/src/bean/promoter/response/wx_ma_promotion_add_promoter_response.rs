//! 对应 Java `cn.binarywang.wx.miniapp.bean.promoter.response.WxMaPromotionAddPromoterResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::promoter::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaPromotionAddPromoterResponse {
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "total_cnt", default)]
    pub total_cnt: i64,
    #[serde(rename = "fail_cnt", default)]
    pub fail_cnt: i64,
    #[serde(rename = "fail_list", default)]
    pub fail_list: Vec<Promoter>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Promoter {
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "role_id", default)]
    pub role_id: i64,
    #[serde(rename = "retail_id", default)]
    pub retail_id: String,
    #[serde(rename = "extra_info", default)]
    pub extra_info: String,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
}
