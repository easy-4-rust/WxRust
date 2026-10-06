//! 对应 Java `com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::QualificationInfo;

/// 微信小店 CategoryDetailResult 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryDetailResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Info>,
    #[serde(rename = "attr", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr: Option<Attr>,
    #[serde(rename = "product_qua_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_qua_list: Option<Vec<QualificationInfo>>,
}

/// 微信小店 Info 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Info {
    #[serde(rename = "cat_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 微信小店 Attr 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attr {
    #[serde(rename = "shop_no_shipment", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_no_shipment: Option<bool>,
    #[serde(rename = "access_permit_required", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_permit_required: Option<bool>,
    #[serde(rename = "pre_sale", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_sale: Option<bool>,
    #[serde(rename = "seven_day_return", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_day_return: Option<bool>,
    #[serde(rename = "brand_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brands: Option<Vec<BrandInfo>>,
    #[serde(rename = "deposit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit: Option<i64>,
    #[serde(rename = "product_attr_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_attrs: Option<Vec<ProductAttr>>,
    #[serde(rename = "sale_attr_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_attrs: Option<Vec<ProductAttr>>,
    #[serde(rename = "transactionfee_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_info: Option<FeeInfo>,
    #[serde(rename = "coupon_rule", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_rule: Option<CouponRule>,
    #[serde(rename = "floor_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor_price: Option<i64>,
    #[serde(rename = "confirm_receipt_days", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_receipt_days: Option<Vec<String>>,
    #[serde(rename = "is_limit_brand", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_brand: Option<bool>,
    #[serde(rename = "product_requirement", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_requirement: Option<ProductRequirement>,
    #[serde(rename = "size_chart", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_chart: Option<SizeChart>,
    #[serde(rename = "is_confidence_require_bad_must_pay", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_require_bad_must_pay: Option<bool>,
    #[serde(rename = "product_qua_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_qua_list: Option<Vec<QualificationInfo>>,
}

/// 微信小店 BrandInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandInfo {
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 微信小店 ProductAttr 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAttr {
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "type_v2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_v2: Option<String>,
    #[serde(rename = "value", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "is_required", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "hint", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hint: Option<String>,
    #[serde(rename = "append_allowed", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub append_allowed: Option<bool>,
}

/// 微信小店 FeeInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FeeInfo {
    #[serde(rename = "basis_point", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_point: Option<i32>,
    #[serde(rename = "original_basis_point", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_basis_point: Option<i32>,
    #[serde(rename = "incentive_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incentive_type: Option<i32>,
}

/// 微信小店 CouponRule 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponRule {
    #[serde(rename = "discount_ratio_limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_coupon: Option<i32>,
    #[serde(rename = "discount_limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon_type: Option<i32>,
}

/// 微信小店 ProductRequirement 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductRequirement {
    #[serde(rename = "product_title_requirement", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title_requirement: Option<String>,
    #[serde(rename = "product_img_requirement", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_img_requirement: Option<String>,
    #[serde(rename = "product_desc_requirement", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_desc_requirement: Option<String>,
}

/// 微信小店 SizeChart 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SizeChart {
    #[serde(rename = "is_support", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support: Option<bool>,
    #[serde(rename = "item_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_list: Option<Vec<SizeChartItem>>,
}

/// 微信小店 SizeChartItem 数据类型；对应 Java com.binarywang.wxjava.store.bean.category.CategoryDetailResult.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SizeChartItem {
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "unit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "format", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "is_required", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}
