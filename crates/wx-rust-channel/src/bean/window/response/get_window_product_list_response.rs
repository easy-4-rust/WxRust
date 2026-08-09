//! 对应 Java `me.chanjar.weixin.channel.bean.window.response.GetWindowProductListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::window::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetWindowProductListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "products", default)]
    pub products: Vec<ProductInfo>,
    #[serde(rename = "last_buffer", default)]
    pub last_buffer: String,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
}
