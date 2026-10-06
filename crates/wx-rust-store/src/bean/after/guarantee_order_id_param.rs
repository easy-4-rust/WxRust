//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeOrderIdParam.java`。

/// 保障单号参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderIdParam {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
}
