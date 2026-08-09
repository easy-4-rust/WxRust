//! 对应 Java `me.chanjar.weixin.channel.bean.product.SpuGetResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuGetResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "product", default)]
    pub product: SpuInfo,
    #[serde(rename = "edit_product", default)]
    pub edit_product: SpuInfo,
    #[serde(rename = "sale_limit_info", default)]
    pub sale_limit_info: ProductSaleLimitInfo,
}
