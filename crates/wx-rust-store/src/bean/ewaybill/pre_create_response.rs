//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.PreCreateResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 PreCreateResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.PreCreateResponse.java。
pub struct PreCreateResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 预取号结果
    #[serde(rename = "ewaybill_order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ewaybill_order_id: Option<String>,
}
