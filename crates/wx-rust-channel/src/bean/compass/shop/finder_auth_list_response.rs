//! 对应 Java `me.chanjar.weixin.channel.bean.compass.shop.FinderAuthListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::compass::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinderAuthListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "main_finder_id", default)]
    pub main_finder_id: String,
    #[serde(rename = "authorized_finder_id_list", default)]
    pub authorized_finder_id_list: Vec<String>,
}
