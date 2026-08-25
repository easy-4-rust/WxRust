//! 对应 Java `me.chanjar.weixin.channel.bean.talent.TalentOrderListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 佣金单列表
    #[serde(rename = "order_list", default)]
    pub order_list: Vec<TalentOrderInfo>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderInfo {
    /// 佣金单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    pub product_id: String,
}
