//! 对应 Java `me.chanjar.weixin.channel.bean.league.supplier.CommissionOrderListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommissionOrderListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "list", default)]
    pub list: Vec<ProductIdInfo>,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductIdInfo {
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    #[serde(rename = "sku_id", default)]
    pub sku_id: String,
}
