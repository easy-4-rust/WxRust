//! 对应 Java `me.chanjar.weixin.channel.bean.ewaybill.CreateOrderRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderRequest {
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

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillAddress {
    /// 姓名
    #[serde(rename = "name", default)]
    pub name: String,
    /// 手机号
    #[serde(rename = "phone", default)]
    pub phone: String,
    /// 省
    #[serde(rename = "province", default)]
    pub province: String,
    /// 市
    #[serde(rename = "city", default)]
    pub city: String,
    /// 区
    #[serde(rename = "district", default)]
    pub district: String,
    /// 详细地址
    #[serde(rename = "address", default)]
    pub address: String,
}
