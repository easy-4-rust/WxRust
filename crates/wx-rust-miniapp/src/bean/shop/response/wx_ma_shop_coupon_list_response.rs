//! 对应 Java `cn.binarywang.wx.miniapp.bean.shop.response.WxMaShopCouponListResponse.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::shop::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaShopCouponListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "total_num", default)]
    pub total_num: i64,
    #[serde(rename = "result_list", default)]
    pub result_list: Vec<ResponseCouponResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResponseCouponResult {
    #[serde(rename = "coupon", default)]
    pub coupon: WxMaShopCouponInfo,
}
