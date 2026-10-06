//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipAssignRequest.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DropshipAssignRequest；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipAssignRequest.java。
pub struct DropshipAssignRequest {
    /// 订单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
}
