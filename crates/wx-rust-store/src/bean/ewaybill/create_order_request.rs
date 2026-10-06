//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.CreateOrderRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CreateOrderRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.CreateOrderRequest.java。
pub struct CreateOrderRequest {
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

/// 微信小店 WaybillAddress 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.CreateOrderRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WaybillAddress {
    /// 姓名
    #[serde(rename = "name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 手机号
    #[serde(rename = "phone", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 省
    #[serde(rename = "province", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 市
    #[serde(rename = "city", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 区
    #[serde(rename = "district", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub district: Option<String>,
    /// 详细地址
    #[serde(rename = "address", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
