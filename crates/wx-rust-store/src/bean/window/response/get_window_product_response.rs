//! 对应 Java `com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GetWindowProductResponse；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
pub struct GetWindowProductResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "product", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}

/// 微信小店 Product 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(rename = "third_category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_category_id: Option<String>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "market_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_price: Option<i64>,
    #[serde(rename = "selling_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_price: Option<i64>,
    #[serde(rename = "stock", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(rename = "page_path", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_path: Option<PagePath>,
    #[serde(rename = "platform_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_id: Option<i64>,
    #[serde(rename = "platform_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_name: Option<String>,
    #[serde(rename = "is_hide_for_window", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hide_for_window: Option<bool>,
    #[serde(rename = "banned", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned: Option<bool>,
    #[serde(rename = "banned_details", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_details: Option<BannedDetails>,
    #[serde(rename = "branch_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_info: Option<BranchInfo>,
    #[serde(rename = "limit_discount_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_discount_info: Option<LimitDiscountInfo>,
}

/// 微信小店 PagePath 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagePath {
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    #[serde(rename = "half_page_path", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub half_page_path: Option<String>,
    #[serde(rename = "full_page_path", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page_path: Option<String>,
}

/// 微信小店 BannedDetails 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BannedDetails {
    #[serde(rename = "reason", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<i32>,
    #[serde(rename = "need_apply_category_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_apply_category_id: Option<String>,
    #[serde(rename = "need_apply_category_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_apply_category_name: Option<String>,
}

/// 微信小店 BranchInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchInfo {
    #[serde(rename = "branch_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_id: Option<i64>,
    #[serde(rename = "branch_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "branch_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_status: Option<i32>,
}

/// 微信小店 LimitDiscountInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitDiscountInfo {
    #[serde(rename = "is_effect", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_effect: Option<bool>,
    #[serde(rename = "discount_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_price: Option<i64>,
    #[serde(rename = "end_time_ms", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_ms: Option<String>,
    #[serde(rename = "stock", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
}
