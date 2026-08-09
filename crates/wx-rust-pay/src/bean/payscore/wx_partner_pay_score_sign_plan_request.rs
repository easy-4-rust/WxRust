//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.WxPartnerPayScoreSignPlanRequest.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPartnerPayScoreSignPlanRequest {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_order_no"
    )]
    pub out_order_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "service_id"
    )]
    pub service_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "service_introduction"
    )]
    pub service_introduction: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "time_range"
    )]
    pub time_range: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "location")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "risk_fund")]
    pub risk_fund: Option<RiskFund>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "need_user_confirm"
    )]
    pub need_user_confirm: Option<bool>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "profit_sharing"
    )]
    pub profit_sharing: Option<bool>,
    #[serde(default, rename = "post_payments")]
    pub post_payments: Vec<PostPayment>,
    #[serde(default, rename = "post_discounts")]
    pub post_discounts: Vec<PostDiscount>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "reason")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "goods_tag")]
    pub goods_tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub r#type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "detail")]
    pub detail: Option<SyncDetail>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_code"
    )]
    pub authorization_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "complete_time"
    )]
    pub complete_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "device")]
    pub device: Option<Device>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_openid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_name")]
    pub plan_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_duration"
    )]
    pub plan_duration: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deduction_quantity"
    )]
    pub deduction_quantity: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_original_price"
    )]
    pub total_original_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_actual_price"
    )]
    pub total_actual_price: Option<i32>,
    #[serde(default, rename = "plan_detail_list")]
    pub plan_detail_list: Vec<PayScorePlanDetailRequest>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_plan_no"
    )]
    pub merchant_plan_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sign_plan_id"
    )]
    pub sign_plan_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_detail_no"
    )]
    pub plan_detail_no: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_id")]
    pub plan_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_sign_plan_no"
    )]
    pub merchant_sign_plan_no: Option<String>,
    #[serde(default, rename = "sign_plan_detail")]
    pub sign_plan_detail: Vec<UserSignPlanDetailMerchatNo>,
}

impl WxPartnerPayScoreSignPlanRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxPartnerPayScoreSignPlanRequest 序列化失败: {e}"))
    }
}
