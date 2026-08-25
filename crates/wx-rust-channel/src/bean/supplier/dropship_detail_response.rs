//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.DropshipDetailResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::dropship_info::DropshipInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 代发单详情
    #[serde(rename = "dropship_info", default)]
    pub dropship_info: DropshipInfo,
}
