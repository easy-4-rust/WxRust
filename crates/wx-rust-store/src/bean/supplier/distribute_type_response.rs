//! 对应 Java `com.binarywang.wxjava.store.bean.supplier.DistributeTypeResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 DistributeTypeResponse；对应 Java com.binarywang.wxjava.store.bean.supplier.DistributeTypeResponse.java。
pub struct DistributeTypeResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 分配方式：1-手动分配，2-全店自动分配，3-按商品自动分配
    #[serde(rename = "distribute_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribute_type: Option<i32>,
    /// 供货商 ID（全店自动分配时有值）
    #[serde(rename = "supplier_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplier_id: Option<String>,
    /// 上游字段 supplier_info。
    #[serde(
        rename = "supplier_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supplier_info: Option<super::SupplierInfo>,
}
