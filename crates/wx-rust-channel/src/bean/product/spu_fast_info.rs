//! 对应 Java `me.chanjar.weixin.channel.bean.product.SpuFastInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuFastInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "skus", default)]
    pub skus: Vec<SkuFastInfo>,
    #[serde(rename = "spu_code", default)]
    pub spu_code: String,
    #[serde(rename = "limit_info", default)]
    pub limit_info: LimitInfo,
    #[serde(rename = "express_info", default)]
    pub express_info: ExpressInfo,
    #[serde(rename = "extra_service", default)]
    pub extra_service: ExtraServiceInfo,
    #[serde(rename = "deliver_method", default)]
    pub deliver_method: i32,
    #[serde(rename = "timing_onsale_info", default)]
    pub timing_on_sale_info: TimingOnSaleInfo,
}
