//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.OrderDetailResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OrderDetailResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.OrderDetailResponse.java。
pub struct OrderDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 运单 ID
    #[serde(rename = "ewaybill_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_id: Option<String>,
    /// 运单状态
    #[serde(rename = "status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
