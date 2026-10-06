//! 对应 Java `com.binarywang.wxjava.store.bean.qic.InspectCodeResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 InspectCodeResponse；对应 Java com.binarywang.wxjava.store.bean.qic.InspectCodeResponse.java。
pub struct InspectCodeResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 质检码 URL
    #[serde(rename = "inspect_code_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspect_code_url: Option<String>,
    /// 上游字段 data。
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<InspectCodeResponseDataPayload>,
}

/// 微信小店嵌套数据。对应 Java: qic.InspectCodeResponse#DataPayload
/// 微信小店 InspectCodeResponseDataPayload 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.InspectCodeResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectCodeResponseDataPayload {
    /// 上游字段 backupDeliveryId。
    #[serde(
        rename = "backupDeliveryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_delivery_id: Option<String>,
    /// 上游字段 backupDeliveryName。
    #[serde(
        rename = "backupDeliveryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_delivery_name: Option<String>,
    /// 上游字段 boxDTOList。
    #[serde(
        rename = "boxDTOList",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub box_info_list: Option<Vec<InspectCodeResponseBoxInfo>>,
    /// 上游字段 channelAppId。
    #[serde(
        rename = "channelAppId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_app_id: Option<String>,
    /// 上游字段 deliveryId。
    #[serde(
        rename = "deliveryId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_id: Option<String>,
    /// 上游字段 deliveryName。
    #[serde(
        rename = "deliveryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub delivery_name: Option<String>,
    /// 上游字段 embedGoodsMaterial。
    #[serde(
        rename = "embedGoodsMaterial",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub embed_goods_material: Option<String>,
    /// 上游字段 goodsDesc。
    #[serde(rename = "goodsDesc", skip_serializing_if = "Option::is_none")]
    pub goods_desc: Option<String>,
    /// 上游字段 expressMerge。
    #[serde(
        rename = "expressMerge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub express_merge: Option<bool>,
    /// 上游字段 goodsMainMaterial。
    #[serde(
        rename = "goodsMainMaterial",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goods_main_material: Option<String>,
    /// 上游字段 goodsName。
    #[serde(rename = "goodsName", skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    /// 上游字段 goodsNum。
    #[serde(rename = "goodsNum", skip_serializing_if = "Option::is_none")]
    pub goods_num: Option<i32>,
    /// 上游字段 goodsPartsMaterial。
    #[serde(
        rename = "goodsPartsMaterial",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub goods_parts_material: Option<String>,
    /// 上游字段 inspectBaseId。
    #[serde(
        rename = "inspectBaseId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_base_id: Option<String>,
    /// 上游字段 inspectBaseName。
    #[serde(
        rename = "inspectBaseName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_base_name: Option<String>,
    /// 上游字段 inspectCode。
    #[serde(
        rename = "inspectCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_code: Option<String>,
    /// 上游字段 inspectOrgId。
    #[serde(
        rename = "inspectOrgId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_id: Option<String>,
    /// 上游字段 inspectOrgName。
    #[serde(
        rename = "inspectOrgName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_name: Option<String>,
    /// 上游字段 inspectOrgShortName。
    #[serde(
        rename = "inspectOrgShortName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inspect_org_short_name: Option<String>,
    /// 上游字段 merchantName。
    #[serde(
        rename = "merchantName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merchant_name: Option<String>,
    /// 上游字段 orderId。
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 urgentOrder。
    #[serde(
        rename = "urgentOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub urgent_order: Option<bool>,
    /// 上游字段 printInfo。
    #[serde(rename = "printInfo", skip_serializing_if = "Option::is_none")]
    pub print_info: Option<String>,
    /// 上游字段 needLabel。
    #[serde(rename = "needLabel", skip_serializing_if = "Option::is_none")]
    pub need_label: Option<bool>,
}

/// 微信小店嵌套数据。对应 Java: qic.InspectCodeResponse#BoxInfo
/// 微信小店 InspectCodeResponseBoxInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.qic.InspectCodeResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InspectCodeResponseBoxInfo {
    /// 上游字段 boxId。
    #[serde(rename = "boxId", skip_serializing_if = "Option::is_none")]
    pub box_id: Option<i64>,
    /// 上游字段 boxName。
    #[serde(rename = "boxName", skip_serializing_if = "Option::is_none")]
    pub box_name: Option<String>,
    /// 上游字段 boxNum。
    #[serde(rename = "boxNum", skip_serializing_if = "Option::is_none")]
    pub box_num: Option<i32>,
}
