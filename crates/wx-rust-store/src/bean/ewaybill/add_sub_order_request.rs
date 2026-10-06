//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.AddSubOrderRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 AddSubOrderRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.AddSubOrderRequest.java。
pub struct AddSubOrderRequest {
    /// 运单 ID
    #[serde(rename = "ewaybill_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_id: Option<String>,
    /// 子件信息
    #[serde(rename = "sub_order_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_order_list: Option<Vec<SubOrderInfo>>,
}

/// 微信小店 SubOrderInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.ewaybill.AddSubOrderRequest.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubOrderInfo {
    /// 子件运单 ID
    #[serde(rename = "sub_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_order_id: Option<String>,
}
