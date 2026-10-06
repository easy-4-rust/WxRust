//! 对应 Java `com.binarywang.wxjava.store.bean.after.GuaranteeModifyRequest.java`。

/// 商家协商保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeModifyRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guarantee_order_id: Option<String>,
    /// 商品破损程度。
    #[serde(rename = "bad_level", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_level: Option<i32>,
    /// 商家协商备注。
    #[serde(rename = "merchant_remark", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_remark: Option<String>,
}
