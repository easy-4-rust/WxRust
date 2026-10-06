//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DropshipResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DropshipResponse；对应 Java com.binarywang.wxjava.store.bean.supplier.DropshipResponse.java。
pub struct DropshipResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 supplier_id。
    #[serde(
        rename = "supplier_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_id: Option<String>,
    /// 上游字段 dropship_id。
    #[serde(
        rename = "dropship_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dropship_id: Option<String>,
}
