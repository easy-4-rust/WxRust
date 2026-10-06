//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.PrintOrderRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 PrintOrderRequest；对应 Java com.binarywang.wxjava.store.bean.ewaybill.PrintOrderRequest.java。
pub struct PrintOrderRequest {
    /// 快递公司标识。对应 Java: PrintOrderRequest#deliveryId。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_id: Option<String>,
    /// 运单标识。对应 Java: PrintOrderRequest#waybillId。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waybill_id: Option<String>,
    /// 是否重新打印。对应 Java: PrintOrderRequest#rePrint。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re_print: Option<bool>,
    /// 运单 ID
    #[serde(rename = "ewaybill_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_id: Option<String>,
}
