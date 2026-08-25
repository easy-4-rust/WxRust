//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.DeliveryListResponse.java`。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 快递公司列表
    #[serde(rename = "delivery_list", default)]
    pub delivery_list: Vec<DeliveryInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryInfo {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    /// 快递公司名称
    #[serde(rename = "delivery_name", default)]
    pub delivery_name: String,
}
