//! 团购券信息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.voucher.VoucherInfo.java`。

use serde::{Deserialize, Serialize};

/// 团购券信息（对应 Java `VoucherInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct VoucherInfo {
    /// 券code（对应 Java `code`）。
    #[serde(rename = "code", default)]
    pub code: Option<String>,
    /// 劵码类型，1商户实时code 2户预存 3平台生成（对应 Java `codeType`）。
    #[serde(
        rename = "code_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub code_type: Option<i32>,
    /// 券状态（对应 Java `status`）。
    #[serde(
        rename = "status",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub status: Option<i32>,
    /// 发放时间，时间戳（对应 Java `sendTime`）。
    #[serde(
        rename = "send_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub send_time: Option<i64>,
    /// 最近更新时间，时间戳（对应 Java `updateTime`）。
    #[serde(
        rename = "update_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub update_time: Option<i64>,
    /// 核销生效时间，时间戳（对应 Java `startTime`）。
    #[serde(
        rename = "start_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub start_time: Option<i64>,
    /// 核销结束时间，时间戳（对应 Java `endTime`）。
    #[serde(
        rename = "end_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub end_time: Option<i64>,
    /// 核销时间，时间戳。次卡时不返回此字段（对应 Java `consumeTime`）。
    #[serde(
        rename = "consume_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub consume_time: Option<i64>,
    /// 退券时间，时间戳。次卡时不返回此字段（对应 Java `refundTime`）。
    #[serde(
        rename = "refund_time",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i64"
    )]
    pub refund_time: Option<i64>,
    /// 核销门店名称（对应 Java `consumeStoreName`）。
    #[serde(rename = "consume_store_name", default)]
    pub consume_store_name: Option<String>,
    /// 券类型（对应 Java `voucherType`）。
    #[serde(
        rename = "voucher_type",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub voucher_type: Option<i32>,
    /// 券的售卖价格（分）（对应 Java `voucherBuyAmount`）。
    #[serde(
        rename = "voucher_buy_amount",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub voucher_buy_amount: Option<i32>,
    /// 券市场金额（分）（对应 Java `voucherActualAmount`）。
    #[serde(
        rename = "voucher_actual_amount",
        default,
        deserialize_with = "crate::bean::message::serde_helpers::opt_string_or_i32"
    )]
    pub voucher_actual_amount: Option<i32>,
    /// 用户手机号（对应 Java `telPhoneNo`）。
    #[serde(rename = "telphone_no", default)]
    pub tel_phone_no: Option<String>,
    /// 商品id（对应 Java `productId`）。
    #[serde(rename = "product_id", default)]
    pub product_id: Option<String>,
    /// 商品下的skuId（对应 Java `skuId`）。
    #[serde(rename = "sku_id", default)]
    pub sku_id: Option<String>,
    /// 购买券的订单id（对应 Java `orderId`）。
    #[serde(rename = "order_id", default)]
    pub order_id: Option<String>,
    /// 用户在商家品牌appid下的openid（对应 Java `openId`）。
    #[serde(rename = "openid", default)]
    pub open_id: Option<String>,
}
