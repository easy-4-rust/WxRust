//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.PreCreateRequest.java`。

use super::create_order_request::WaybillAddress;

/// 微信小店 PreCreateRequest 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.PreCreateRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreCreateRequest {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    /// 订单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 收件人信息
    #[serde(rename = "recv_addr", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recv_addr: Option<WaybillAddress>,
    /// 寄件人信息
    #[serde(rename = "send_addr", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_addr: Option<WaybillAddress>,
}
