//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java`。

#[allow(unused_imports)]
use super::talent_order_list_response::TalentOrderInfo;

/// 微信小店 TalentOrderDetailResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 佣金单详情
    #[serde(rename = "order_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_detail: Option<TalentOrderInfo>,
    /// 上游字段 base_info。
    #[serde(rename = "base_info", skip_serializing_if = "Option::is_none")]
    pub base_info: Option<TalentOrderDetailResponseBaseInfo>,
    /// 上游字段 commission_info。
    #[serde(
        rename = "commission_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub commission_info: Option<TalentOrderDetailResponseCommissionInfo>,
    /// 上游字段 channel_info。
    #[serde(
        rename = "channel_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_info: Option<TalentOrderDetailResponseStoreInfo>,
    /// 上游字段 promotion_head_supplier_info。
    #[serde(
        rename = "promotion_head_supplier_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub promotion_head_supplier_info: Option<TalentOrderDetailResponsePromotionHeadSupplierInfo>,
    /// 上游字段 product_info。
    #[serde(
        rename = "product_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_info: Option<TalentOrderDetailResponseProductInfo>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#BaseInfo
/// 微信小店 TalentOrderDetailResponseBaseInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponseBaseInfo {
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 spu_id。
    #[serde(rename = "spu_id", skip_serializing_if = "Option::is_none")]
    pub spu_id: Option<String>,
    /// 上游字段 sku_id。
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    /// 上游字段 special_id。
    #[serde(
        rename = "special_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_id: Option<String>,
    /// 上游字段 order_status。
    #[serde(
        rename = "order_status",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_status: Option<i32>,
    /// 上游字段 actual_payment。
    #[serde(
        rename = "actual_payment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_payment: Option<String>,
    /// 上游字段 order_create_time。
    #[serde(
        rename = "order_create_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_create_time: Option<i64>,
    /// 上游字段 order_update_time。
    #[serde(
        rename = "order_update_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_update_time: Option<i64>,
    /// 上游字段 buyer_info。
    #[serde(
        rename = "buyer_info",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_info: Option<TalentOrderDetailResponseBuyerInfo>,
    /// 上游字段 order_pay_time。
    #[serde(
        rename = "order_pay_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_pay_time: Option<i64>,
    /// 上游字段 settle_payment。
    #[serde(
        rename = "settle_payment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub settle_payment: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#BuyerInfo
/// 微信小店 TalentOrderDetailResponseBuyerInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponseBuyerInfo {
    /// 上游字段 open_id。
    #[serde(rename = "open_id", skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 上游字段 union_id。
    #[serde(rename = "union_id", skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#CommissionInfo
/// 微信小店 TalentOrderDetailResponseCommissionInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponseCommissionInfo {
    /// 上游字段 state。
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<i32>,
    /// 上游字段 ratio。
    #[serde(rename = "ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// 上游字段 expect_settle_time。
    #[serde(
        rename = "expect_settle_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expect_settle_time: Option<i64>,
    /// 上游字段 expect_settlement。
    #[serde(
        rename = "expect_settlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub expect_settlement: Option<String>,
    /// 上游字段 actual_settle_time。
    #[serde(
        rename = "actual_settle_time",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_settle_time: Option<i64>,
    /// 上游字段 actual_settlement。
    #[serde(
        rename = "actual_settlement",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub actual_settlement: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#StoreInfo
/// 微信小店 TalentOrderDetailResponseStoreInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponseStoreInfo {
    /// 上游字段 channel_type。
    #[serde(
        rename = "channel_type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_type: Option<i32>,
    /// 上游字段 channel_id。
    #[serde(
        rename = "channel_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_id: Option<String>,
    /// 上游字段 channel_name。
    #[serde(
        rename = "channel_name",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#PromotionHeadSupplierInfo
/// 微信小店 TalentOrderDetailResponsePromotionHeadSupplierInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponsePromotionHeadSupplierInfo {
    /// 上游字段 id。
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 上游字段 name。
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 上游字段 ratio。
    #[serde(rename = "ratio", skip_serializing_if = "Option::is_none")]
    pub ratio: Option<String>,
    /// 上游字段 fee。
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderDetailResponse#ProductInfo
/// 微信小店 TalentOrderDetailResponseProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderDetailResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderDetailResponseProductInfo {
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上游字段 thumb_img。
    #[serde(rename = "thumb_img", skip_serializing_if = "Option::is_none")]
    pub thumb_img: Option<String>,
}
