//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.CoopProductResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CoopProductResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "item", default)]
    pub item: Item,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "product_info", default)]
    pub product_info: ProductInfo,
    #[serde(rename = "commission_info", default)]
    pub commission_info: CommissionInfo,
}
