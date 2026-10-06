//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.DeliveryListResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DeliveryListResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.DeliveryListResponse.java。
pub struct DeliveryListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 快递公司列表
    #[serde(rename = "delivery_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_list: Option<Vec<DeliveryInfo>>,
}

/// 微信小店 DeliveryInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.DeliveryListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryInfo {
    /// 快递公司 ID
    #[serde(rename = "delivery_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// 快递公司名称
    #[serde(rename = "delivery_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_name: Option<String>,
}
