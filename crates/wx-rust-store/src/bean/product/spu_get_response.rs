//! 对应 Java `com.binarywang.wxjava.store.bean.product.SpuGetResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{ProductSaleLimitInfo, SpuInfo};

/// 微信小店 SpuGetResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.SpuGetResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SpuGetResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "product", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<SpuInfo>,
    #[serde(rename = "edit_product", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_product: Option<SpuInfo>,
    #[serde(rename = "sale_limit_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_limit_info: Option<ProductSaleLimitInfo>,
}
