//! 对应 Java `me.chanjar.weixin.channel.bean.category.CategoryDetailResult.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CategoryDetailResult {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "info", default)]
    pub info: Info,
    #[serde(rename = "attr", default)]
    pub attr: Attr,
    #[serde(rename = "product_qua_list", default)]
    pub product_qua_list: Vec<QualificationInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Info {
    #[serde(rename = "cat_id", default)]
    pub id: String,
    #[serde(rename = "name", default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attr {
    #[serde(rename = "shop_no_shipment", default)]
    pub shop_no_shipment: bool,
    #[serde(rename = "access_permit_required", default)]
    pub access_permit_required: bool,
    #[serde(rename = "pre_sale", default)]
    pub pre_sale: bool,
    #[serde(rename = "seven_day_return", default)]
    pub seven_day_return: bool,
    #[serde(rename = "brand_list", default)]
    pub brands: Vec<BrandInfo>,
    #[serde(rename = "deposit", default)]
    pub deposit: i64,
    #[serde(rename = "product_attr_list", default)]
    pub product_attrs: Vec<ProductAttr>,
    #[serde(rename = "sale_attr_list", default)]
    pub sale_attrs: Vec<ProductAttr>,
    #[serde(rename = "transactionfee_info", default)]
    pub fee_info: FeeInfo,
    #[serde(rename = "coupon_rule", default)]
    pub coupon_rule: CouponRule,
    #[serde(rename = "floor_price", default)]
    pub floor_price: i64,
    #[serde(rename = "confirm_receipt_days", default)]
    pub confirm_receipt_days: Vec<String>,
    #[serde(rename = "is_limit_brand", default)]
    pub limit_brand: bool,
    #[serde(rename = "product_requirement", default)]
    pub product_requirement: ProductRequirement,
    #[serde(rename = "size_chart", default)]
    pub size_chart: SizeChart,
    #[serde(rename = "is_confidence_require_bad_must_pay", default)]
    pub confidence_require_bad_must_pay: bool,
    #[serde(rename = "product_qua_list", default)]
    pub product_qua_list: Vec<QualificationInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandInfo {
    #[serde(rename = "brand_id", default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductAttr {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "type_v2", default)]
    pub type_v2: String,
    #[serde(rename = "value", default)]
    pub value: String,
    #[serde(rename = "is_required", default)]
    pub required: bool,
    #[serde(rename = "hint", default)]
    pub hint: String,
    #[serde(rename = "append_allowed", default)]
    pub append_allowed: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FeeInfo {
    #[serde(rename = "basis_point", default)]
    pub basis_point: i32,
    #[serde(rename = "original_basis_point", default)]
    pub original_basis_point: i32,
    #[serde(rename = "incentive_type", default)]
    pub incentive_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CouponRule {
    #[serde(rename = "discount_ratio_limit", default)]
    pub support_coupon: i32,
    #[serde(rename = "discount_limit", default)]
    pub coupon_type: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductRequirement {
    #[serde(rename = "product_title_requirement", default)]
    pub product_title_requirement: String,
    #[serde(rename = "product_img_requirement", default)]
    pub product_img_requirement: String,
    #[serde(rename = "product_desc_requirement", default)]
    pub product_desc_requirement: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SizeChart {
    #[serde(rename = "is_support", default)]
    pub support: bool,
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<SizeChartItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SizeChartItem {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "unit", default)]
    pub unit: String,
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "format", default)]
    pub format: String,
    #[serde(rename = "limit", default)]
    pub limit: String,
    #[serde(rename = "is_required", default)]
    pub required: bool,
}
