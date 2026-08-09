//! 对应 Java `com.github.binarywang.wxpay.bean.notify.MiPayNotifyV3Result.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MiPayNotifyV3Result {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rawData")]
    pub raw_data: Option<OriginNotifyResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "result")]
    pub result: Option<DecryptNotifyResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecryptNotifyResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mix_trade_no"
    )]
    pub mix_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mix_pay_status"
    )]
    pub mix_pay_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "self_pay_status"
    )]
    pub self_pay_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_pay_status"
    )]
    pub med_ins_pay_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "paid_time")]
    pub paid_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "passthrough_response_content"
    )]
    pub passthrough_response_content: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "mix_pay_type"
    )]
    pub mix_pay_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "order_type"
    )]
    pub order_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "pay_for_relatives"
    )]
    pub pay_for_relatives: Option<bool>,
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
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_order_create_time"
    )]
    pub med_ins_order_create_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_order_finish_time"
    )]
    pub med_ins_order_finish_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_gov_fee"
    )]
    pub med_ins_gov_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_self_fee"
    )]
    pub med_ins_self_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_other_fee"
    )]
    pub med_ins_other_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "med_ins_cash_fee"
    )]
    pub med_ins_cash_fee: Option<i64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechat_pay_cash_fee"
    )]
    pub wechat_pay_cash_fee: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_state"
    )]
    pub trade_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "trade_state_desc"
    )]
    pub trade_state_desc: Option<String>,
}
