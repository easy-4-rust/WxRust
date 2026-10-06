//! 对应 Java `com.binarywang.wxjava.store.bean.qic.InspectConfigResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 InspectConfigResponse；对应 Java com.binarywang.wxjava.store.bean.qic.InspectConfigResponse.java。
pub struct InspectConfigResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 是否开通质检仓
    #[serde(rename = "is_opened", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_opened: Option<bool>,
    /// 上游字段 inspect_config。
    #[serde(
        rename = "inspect_config",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_config: Option<InspectConfigResponseInspectConfig>,
}

/// 微信小店嵌套数据。对应 Java: qic.InspectConfigResponse#InspectConfig
/// 微信小店 InspectConfigResponseInspectConfig 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.InspectConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectConfigResponseInspectConfig {
    /// 上游字段 warehouse_id。
    #[serde(
        rename = "warehouse_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warehouse_id: Option<String>,
    /// 上游字段 delivery_address。
    #[serde(
        rename = "delivery_address",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_address: Option<InspectConfigResponseAddress>,
    /// 上游字段 return_address。
    #[serde(
        rename = "return_address",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub return_address: Option<InspectConfigResponseAddress>,
    /// 上游字段 warehouse_name。
    #[serde(
        rename = "warehouse_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warehouse_name: Option<String>,
    /// 上游字段 warehouse_addr。
    #[serde(
        rename = "warehouse_addr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub warehouse_addr: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: qic.InspectConfigResponse#Address
/// 微信小店 InspectConfigResponseAddress 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.InspectConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectConfigResponseAddress {
    /// 上游字段 contact_name。
    #[serde(
        rename = "contact_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_name: Option<String>,
    /// 上游字段 contact_phone。
    #[serde(
        rename = "contact_phone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub contact_phone: Option<String>,
    /// 上游字段 province。
    #[serde(rename = "province", skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 上游字段 city。
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 上游字段 county。
    #[serde(rename = "county", skip_serializing_if = "Option::is_none")]
    pub county: Option<String>,
    /// 上游字段 detail。
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}
