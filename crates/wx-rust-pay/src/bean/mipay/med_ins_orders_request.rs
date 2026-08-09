//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.MedInsOrdersRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MedInsOrdersRequest {
    #[serde(default, rename = "mix_pay_type")]
    pub mix_pay_type: String,
    #[serde(default, rename = "order_type")]
    pub order_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "payer")]
    pub payer: Option<PersonIdentification>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_for_relatives"
    )]
    pub pay_for_relatives: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "relative")]
    pub relative: Option<PersonIdentification>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "serial_no")]
    pub serial_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_order_id"
    )]
    pub pay_order_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_auth_no"
    )]
    pub pay_auth_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "geo_location"
    )]
    pub geo_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "city_id")]
    pub city_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_inst_name"
    )]
    pub med_inst_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_inst_no"
    )]
    pub med_inst_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_order_create_time"
    )]
    pub med_ins_order_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_gov_fee"
    )]
    pub med_ins_gov_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_self_fee"
    )]
    pub med_ins_self_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_other_fee"
    )]
    pub med_ins_other_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_cash_fee"
    )]
    pub med_ins_cash_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechat_pay_cash_fee"
    )]
    pub wechat_pay_cash_fee: Option<i32>,
    #[serde(default, rename = "cash_add_detail")]
    pub cash_add_detail: Vec<CashAddEntity>,
    #[serde(default, rename = "cash_reduce_detail")]
    pub cash_reduce_detail: Vec<CashReduceEntity>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "callback_url"
    )]
    pub callback_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "prepay_id")]
    pub prepay_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "passthrough_request_content"
    )]
    pub passthrough_request_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extends")]
    pub _extends: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "channel_no"
    )]
    pub channel_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_test_env"
    )]
    pub med_ins_test_env: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PersonIdentification {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "name")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "id_digest")]
    pub id_digest: Option<String>,
    #[serde(default, rename = "card_type")]
    pub card_type: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashAddEntity {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cash_add_fee"
    )]
    pub cash_add_fee: Option<i32>,
    #[serde(default, rename = "cash_add_type")]
    pub cash_add_type: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashReduceEntity {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cash_reduce_fee"
    )]
    pub cash_reduce_fee: Option<i32>,
    #[serde(default, rename = "cash_reduce_type")]
    pub cash_reduce_type: String,
}
