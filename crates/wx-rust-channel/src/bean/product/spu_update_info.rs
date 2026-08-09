//! 对应 Java `me.chanjar.weixin.channel.bean.product.SpuUpdateInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AttrInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuUpdateInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "out_product_id", default)]
    pub out_product_id: String,
    #[serde(rename = "skus", default)]
    pub skus: Vec<SkuInfo>,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "sub_title", default)]
    pub sub_title: String,
    #[serde(rename = "head_imgs", default)]
    pub head_imgs: Vec<String>,
    #[serde(rename = "deliver_method", default)]
    pub deliver_method: i32,
    #[serde(rename = "deliver_acct_type", default)]
    pub deliver_acct_type: Vec<i32>,
    #[serde(rename = "desc_info", default)]
    pub desc_info: DescriptionInfo,
    #[serde(rename = "cats", default)]
    pub cats: Vec<SpuCategory>,
    #[serde(rename = "cats_v2", default)]
    pub cats_v2: Vec<SpuCategory>,
    #[serde(rename = "attrs", default)]
    pub attrs: Vec<AttrInfo>,
    #[serde(rename = "spu_code", default)]
    pub spu_code: String,
    #[serde(rename = "brand_id", default)]
    pub brand_id: String,
    #[serde(rename = "qualifications", default)]
    pub qualifications: Vec<String>,
    #[serde(rename = "express_info", default)]
    pub express_info: ExpressInfo,
    #[serde(rename = "aftersale_desc", default)]
    pub after_sale_desc: String,
    #[serde(rename = "limited_info", default)]
    pub limit_info: LimitInfo,
    #[serde(rename = "extra_service", default)]
    pub extra_service: ExtraServiceInfo,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "edit_status", default)]
    pub edit_status: i32,
    #[serde(rename = "min_price", default)]
    pub min_price: i32,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "edit_time", default)]
    pub edit_time: i64,
    #[serde(rename = "product_type", default)]
    pub product_type: i32,
    #[serde(rename = "after_sale_info", default)]
    pub after_sale_info: AfterSaleInfo,
    #[serde(rename = "src_product_id", default)]
    pub src_product_id: String,
    #[serde(rename = "product_qua_infos", default)]
    pub product_qua_infos: Vec<ProductQuaInfo>,
    #[serde(rename = "size_chart", default)]
    pub size_chart: SpuSizeChart,
    #[serde(rename = "short_title", default)]
    pub short_title: String,
    #[serde(rename = "total_sold_num", default)]
    pub total_sold_num: i32,
    #[serde(rename = "release_mode", default)]
    pub release_mode: i32,
    #[serde(rename = "timing_onsale_info", default)]
    pub timing_on_sale_info: TimingOnSaleInfo,
    #[serde(rename = "listing", default)]
    pub listing: i32,
}
