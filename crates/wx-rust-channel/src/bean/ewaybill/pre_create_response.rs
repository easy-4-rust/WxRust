//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.PreCreateResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreCreateResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 预取号结果
    #[serde(rename = "ewaybill_order_id", default)]
    pub ewaybill_order_id: String,
}
