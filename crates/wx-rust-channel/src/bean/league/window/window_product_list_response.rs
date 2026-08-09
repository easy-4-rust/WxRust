//! 对应 Java `me.chanjar.weixin.channel.bean.league.window.WindowProductListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::league::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProductListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "list", default)]
    pub list: Vec<ItemKey>,
    #[serde(rename = "next_offset", default)]
    pub next_offset: i32,
    #[serde(rename = "have_more", default)]
    pub have_more: bool,
    #[serde(rename = "total_num", default)]
    pub total_num: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ItemKey {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "product_id", default)]
    pub product_id: String,
    #[serde(rename = "head_supplier_id", default)]
    pub head_supplier_id: String,
}
