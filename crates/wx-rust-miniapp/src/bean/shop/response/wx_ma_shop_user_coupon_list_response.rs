//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopUserCouponListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopUserCouponListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "total_num", default)]
    pub total_num: i64,
    #[serde(rename = "result_list", default)]
    pub result_list: Vec<UserCouponResultItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponResultItem {
    #[serde(rename = "out_user_coupon_id", default)]
    pub out_user_coupon_id: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "out_coupon_id", default)]
    pub out_coupon_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "update_time", default)]
    pub update_time: i64,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "ext_info", default)]
    pub ext_info: UserCouponExtInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserCouponExtInfo {
    #[serde(rename = "use_time", default)]
    pub use_time: i64,
}
