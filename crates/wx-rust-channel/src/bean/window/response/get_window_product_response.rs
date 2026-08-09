//! 对应 Java `me.chanjar.weixin.channel.bean.window.response.GetWindowProductResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::window::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetWindowProductResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "product", default)]
    pub product: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "img_url", default)]
    pub img_url: String,
    #[serde(rename = "third_category_id", default)]
    pub third_category_id: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "market_price", default)]
    pub market_price: i64,
    #[serde(rename = "selling_price", default)]
    pub selling_price: i64,
    #[serde(rename = "stock", default)]
    pub stock: i64,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "page_path", default)]
    pub page_path: PagePath,
    #[serde(rename = "platform_id", default)]
    pub platform_id: i64,
    #[serde(rename = "platform_name", default)]
    pub platform_name: String,
    #[serde(rename = "is_hide_for_window", default)]
    pub is_hide_for_window: bool,
    #[serde(rename = "banned", default)]
    pub banned: bool,
    #[serde(rename = "banned_details", default)]
    pub banned_details: BannedDetails,
    #[serde(rename = "branch_info", default)]
    pub branch_info: BranchInfo,
    #[serde(rename = "limit_discount_info", default)]
    pub limit_discount_info: LimitDiscountInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagePath {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "half_page_path", default)]
    pub half_page_path: String,
    #[serde(rename = "full_page_path", default)]
    pub full_page_path: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannedDetails {
    #[serde(rename = "reason", default)]
    pub reason: i32,
    #[serde(rename = "need_apply_category_id", default)]
    pub need_apply_category_id: String,
    #[serde(rename = "need_apply_category_name", default)]
    pub need_apply_category_name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchInfo {
    #[serde(rename = "branch_id", default)]
    pub branch_id: i64,
    #[serde(rename = "branch_name", default)]
    pub branch_name: String,
    #[serde(rename = "branch_status", default)]
    pub branch_status: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitDiscountInfo {
    #[serde(rename = "is_effect", default)]
    pub is_effect: bool,
    #[serde(rename = "discount_price", default)]
    pub discount_price: i64,
    #[serde(rename = "end_time_ms", default)]
    pub end_time_ms: String,
    #[serde(rename = "stock", default)]
    pub stock: i64,
}
