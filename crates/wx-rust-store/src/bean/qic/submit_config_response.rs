//! 对应 Java `com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 SubmitConfigResponse；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
pub struct SubmitConfigResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 送检配置模板信息
    #[serde(rename = "submit_config_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_config_info: Option<SubmitConfigInfo>,
    /// 上游字段 submit_config。
    #[serde(
        rename = "submit_config",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub submit_config: Option<SubmitConfigResponseSubmitConfig>,
}

/// 微信小店 SubmitConfigInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigInfo {
    /// 模板 ID
    #[serde(rename = "template_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitConfigResponse#SubmitConfig
/// 微信小店 SubmitConfigResponseSubmitConfig 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponseSubmitConfig {
    /// 上游字段 delivery_list。
    #[serde(
        rename = "delivery_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_list: Option<Vec<SubmitConfigResponseDelivery>>,
    /// 上游字段 inspect_org_list。
    #[serde(
        rename = "inspect_org_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_list: Option<Vec<SubmitConfigResponseInspectOrg>>,
    /// 上游字段 charge_url。
    #[serde(
        rename = "charge_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_url: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitConfigResponse#Delivery
/// 微信小店 SubmitConfigResponseDelivery 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponseDelivery {
    /// 上游字段 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 上游字段 name。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 上游字段 delivery_products。
    #[serde(
        rename = "delivery_products",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_products: Option<Vec<SubmitConfigResponseDeliveryProduct>>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitConfigResponse#DeliveryProduct
/// 微信小店 SubmitConfigResponseDeliveryProduct 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponseDeliveryProduct {
    /// 上游字段 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// 上游字段 name。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 上游字段 enable_insure。
    #[serde(
        rename = "enable_insure",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_insure: Option<i32>,
    /// 上游字段 insure_type_list。
    #[serde(
        rename = "insure_type_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub insure_type_list: Option<Vec<SubmitConfigResponseInsureType>>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitConfigResponse#InsureType
/// 微信小店 SubmitConfigResponseInsureType 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponseInsureType {
    /// 上游字段 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 上游字段 name。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 上游字段 upper_limit_type。
    #[serde(
        rename = "upper_limit_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_limit_type: Option<i32>,
    /// 上游字段 upper_limit_amount。
    #[serde(
        rename = "upper_limit_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub upper_limit_amount: Option<i64>,
}

/// 微信小店嵌套数据。对应 Java: qic.SubmitConfigResponse#InspectOrg
/// 微信小店 SubmitConfigResponseInspectOrg 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.SubmitConfigResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubmitConfigResponseInspectOrg {
    /// 上游字段 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 上游字段 name。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 上游字段 org_category。
    #[serde(
        rename = "org_category",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub org_category: Option<i32>,
}
