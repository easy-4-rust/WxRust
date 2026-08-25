//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentOrderDetailResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

use super::talent_order_list_response::TalentOrderInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 佣金单详情
    #[serde(rename = "order_detail", default)]
    pub order_detail: TalentOrderInfo,
}
