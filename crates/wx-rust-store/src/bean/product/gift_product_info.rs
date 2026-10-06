//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftProductInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{
    AfterSaleInfo, DescriptionInfo, ExpressInfo, ExtraServiceInfo, LimitInfo, ProductQuaInfo,
    SkuInfo, SpuCategory, SpuSizeChart, TimingOnSaleInfo,
};

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

/// 微信小店 GiftProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftProductInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "out_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_product_id: Option<String>,
    #[serde(rename = "skus", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<SkuInfo>>,
    #[serde(rename = "title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "sub_title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_title: Option<String>,
    #[serde(rename = "head_imgs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_imgs: Option<Vec<String>>,
    #[serde(rename = "deliver_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_method: Option<i32>,
    #[serde(rename = "deliver_acct_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_acct_type: Option<Vec<i32>>,
    #[serde(rename = "desc_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc_info: Option<DescriptionInfo>,
    #[serde(rename = "cats", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cats: Option<Vec<SpuCategory>>,
    #[serde(rename = "cats_v2", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cats_v2: Option<Vec<SpuCategory>>,
    #[serde(rename = "attrs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attrs: Option<Vec<AttrInfo>>,
    #[serde(rename = "spu_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spu_code: Option<String>,
    #[serde(rename = "brand_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand_id: Option<String>,
    #[serde(rename = "qualifications", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifications: Option<Vec<String>>,
    #[serde(rename = "express_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_info: Option<ExpressInfo>,
    #[serde(rename = "aftersale_desc", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_desc: Option<String>,
    #[serde(rename = "limited_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_info: Option<LimitInfo>,
    #[serde(rename = "extra_service", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_service: Option<ExtraServiceInfo>,
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "edit_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_status: Option<i32>,
    #[serde(rename = "min_price", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_price: Option<i32>,
    #[serde(rename = "create_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "edit_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_time: Option<i64>,
    #[serde(rename = "product_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<i32>,
    #[serde(rename = "after_sale_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_info: Option<AfterSaleInfo>,
    #[serde(rename = "src_product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub src_product_id: Option<String>,
    #[serde(rename = "product_qua_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_qua_infos: Option<Vec<ProductQuaInfo>>,
    #[serde(rename = "size_chart", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_chart: Option<SpuSizeChart>,
    #[serde(rename = "short_title", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_title: Option<String>,
    #[serde(rename = "total_sold_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_sold_num: Option<i32>,
    #[serde(rename = "release_mode", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_mode: Option<i32>,
    #[serde(rename = "timing_onsale_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_on_sale_info: Option<TimingOnSaleInfo>,
    #[serde(rename = "listing", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub listing: Option<i32>,
}
