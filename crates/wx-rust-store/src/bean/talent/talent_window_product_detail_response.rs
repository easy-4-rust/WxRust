//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentWindowProductDetailResponse.java`。

#[allow(unused_imports)]
use super::talent_window_product_list_response::TalentWindowProductInfo;

/// 微信小店 TalentWindowProductDetailResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 橱窗商品详情
    #[serde(rename = "product_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_detail: Option<TalentWindowProductInfo>,
    /// 上游字段 product。
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<TalentWindowProductDetailResponseProductDetail>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentWindowProductDetailResponse#ProductDetail
/// 微信小店 TalentWindowProductDetailResponseProductDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductDetailResponseProductDetail {
    /// 上游字段 product_id。
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    /// 上游字段 appid。
    #[serde(rename = "appid", skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 上游字段 out_product_id。
    #[serde(
        rename = "out_product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub out_product_id: Option<String>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上游字段 img_url。
    #[serde(rename = "img_url", skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    /// 上游字段 leaf_category_id。
    #[serde(
        rename = "leaf_category_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leaf_category_id: Option<i64>,
    /// 上游字段 status。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 上游字段 selling_price。
    #[serde(
        rename = "selling_price",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub selling_price: Option<i64>,
    /// 上游字段 stock。
    #[serde(rename = "stock", skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    /// 上游字段 sales。
    #[serde(rename = "sales", skip_serializing_if = "Option::is_none")]
    pub sales: Option<i64>,
    /// 上游字段 is_hide。
    #[serde(rename = "is_hide", skip_serializing_if = "Option::is_none")]
    pub is_hide: Option<bool>,
    /// 上游字段 product_promotion_link。
    #[serde(
        rename = "product_promotion_link",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_promotion_link: Option<String>,
}
