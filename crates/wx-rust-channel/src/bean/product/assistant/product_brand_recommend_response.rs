//! 对应 Java `me.chanjar.weixin.channel.bean.product.assistant.ProductBrandRecommendResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductBrandRecommendResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 推荐品牌列表
    #[serde(rename = "brand_list", default)]
    pub brand_list: Vec<RecommendedBrand>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecommendedBrand {
    /// 品牌 ID
    #[serde(rename = "brand_id", default)]
    pub brand_id: String,
    /// 品牌名称
    #[serde(rename = "brand_name", default)]
    pub brand_name: String,
}
