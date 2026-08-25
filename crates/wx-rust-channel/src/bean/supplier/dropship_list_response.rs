//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.DropshipListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::dropship_info::DropshipInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 代发单列表
    #[serde(rename = "dropship_list", default)]
    pub dropship_list: Vec<DropshipInfo>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
