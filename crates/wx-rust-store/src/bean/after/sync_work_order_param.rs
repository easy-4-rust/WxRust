/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam
/// 微信小店 SyncWorkOrderParam 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SyncWorkOrderParam {
    #[serde(
        rename = "complaint_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub complaint_id: Option<String>,
    #[serde(
        rename = "work_order_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_order_info: Option<WorkOrderInfo>,
}

/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam
/// 微信小店 WorkOrderInfo 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkOrderInfo {
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WorkOrderItem>>,
    #[serde(
        rename = "work_order_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_order_id: Option<String>,
}

/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam
/// 微信小店 WorkOrderItem 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkOrderItem {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(
        rename = "update_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<i64>,
    #[serde(
        rename = "result_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub result_type: Option<i32>,
    #[serde(
        rename = "refund_amount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub refund_amount: Option<i32>,
    #[serde(
        rename = "media_list",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_list: Option<Vec<WorkOrderMedia>>,
}

/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam
/// 微信小店 WorkOrderMedia 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkOrderMedia {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(rename = "picture", skip_serializing_if = "Option::is_none")]
    pub picture: Option<WorkOrderPicture>,
}

/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam
/// 微信小店 WorkOrderPicture 数据类型；对应 Java: com.binarywang.wxjava.store.bean.after.SyncWorkOrderParam。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WorkOrderPicture {
    #[serde(
        rename = "tmp_media_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tmp_media_id: Option<String>,
}
