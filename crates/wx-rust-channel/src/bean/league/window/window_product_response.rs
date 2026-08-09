//! 对应 Java `me.chanjar.weixin.channel.bean.league.window.WindowProductResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;
#[allow(unused_imports)]
use crate::bean::league::SimpleProductInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProductResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "product_detail", default)]
    pub product_detail: ProductDetail,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductDetail {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "product_info", default)]
    pub product_info: SimpleProductInfo,
}
