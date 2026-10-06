//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GiftActivityInfo；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
pub struct GiftActivityInfo {
    /// 活动名称
    #[serde(rename = "activity_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    /// 活动开始时间
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 活动结束时间
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 上游字段 activity_id。
    #[serde(
        rename = "activity_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_id: Option<String>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上游字段 detail。
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<GiftActivityInfoDetail>,
}

/// 微信小店嵌套数据。对应 Java: product.GiftActivityInfo#Detail
/// 微信小店 GiftActivityInfoDetail 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfoDetail {
    /// 上游字段 show_scene。
    #[serde(
        rename = "show_scene",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_scene: Option<i32>,
    /// 上游字段 receive_limit。
    #[serde(
        rename = "receive_limit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub receive_limit: Option<GiftActivityInfoReceiveLimit>,
    /// 上游字段 main_products。
    #[serde(
        rename = "main_products",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub main_products: Option<Vec<GiftActivityInfoMainProduct>>,
    /// 上游字段 gift_set。
    #[serde(rename = "gift_set", skip_serializing_if = "Option::is_none")]
    pub gift_set: Option<GiftActivityInfoGiftSet>,
}

/// 微信小店嵌套数据。对应 Java: product.GiftActivityInfo#ReceiveLimit
/// 微信小店 GiftActivityInfoReceiveLimit 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfoReceiveLimit {
    /// 上游字段 is_limited。
    #[serde(
        rename = "is_limited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub limited: Option<bool>,
    /// 上游字段 limit_num。
    #[serde(rename = "limit_num", skip_serializing_if = "Option::is_none")]
    pub limit_num: Option<i32>,
}

/// 微信小店嵌套数据。对应 Java: product.GiftActivityInfo#MainProduct
/// 微信小店 GiftActivityInfoMainProduct 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfoMainProduct {
    /// 上游字段 product_id。
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: product.GiftActivityInfo#GiftSet
/// 微信小店 GiftActivityInfoGiftSet 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfoGiftSet {
    /// 上游字段 gift_items。
    #[serde(
        rename = "gift_items",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gift_items: Option<Vec<GiftActivityInfoGiftItem>>,
    /// 上游字段 gift_set_num。
    #[serde(
        rename = "gift_set_num",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub gift_set_num: Option<i32>,
}

/// 微信小店嵌套数据。对应 Java: product.GiftActivityInfo#GiftItem
/// 微信小店 GiftActivityInfoGiftItem 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftActivityInfoGiftItem {
    /// 上游字段 gift_id。
    #[serde(rename = "gift_id", skip_serializing_if = "Option::is_none")]
    pub gift_id: Option<String>,
    /// 上游字段 give_num。
    #[serde(rename = "give_num", skip_serializing_if = "Option::is_none")]
    pub give_num: Option<i32>,
}
