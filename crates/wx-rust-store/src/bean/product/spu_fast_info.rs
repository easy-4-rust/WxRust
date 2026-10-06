//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuFastInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{ExpressInfo, ExtraServiceInfo, LimitInfo, SkuFastInfo, TimingOnSaleInfo};

/// 微信小店 SpuFastInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuFastInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuFastInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "skus", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skus: Option<Vec<SkuFastInfo>>,
    #[serde(rename = "spu_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spu_code: Option<String>,
    #[serde(rename = "limit_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_info: Option<LimitInfo>,
    #[serde(rename = "express_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub express_info: Option<ExpressInfo>,
    #[serde(rename = "extra_service", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_service: Option<ExtraServiceInfo>,
    #[serde(rename = "deliver_method", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_method: Option<i32>,
    #[serde(rename = "timing_onsale_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing_on_sale_info: Option<TimingOnSaleInfo>,
}
