//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "after_sale_order_id_list", default)]
    pub ids: Vec<String>,
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    #[serde(rename = "has_more", default)]
    pub has_more: bool,
}
