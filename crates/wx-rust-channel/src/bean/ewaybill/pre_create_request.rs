//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.PreCreateRequest.java`。

#[allow(unused_imports)]
use super::*;

use super::create_order_request::WaybillAddress;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreCreateRequest {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    pub delivery_id: String,
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    /// 订单号
    #[serde(rename = "order_id", default)]
    pub order_id: String,
    /// 收件人信息
    #[serde(rename = "recv_addr", default)]
    pub recv_addr: WaybillAddress,
    /// 寄件人信息
    #[serde(rename = "send_addr", default)]
    pub send_addr: WaybillAddress,
}
