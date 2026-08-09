//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.WxPartnerPayScoreSignPlanResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPartnerPayScoreSignPlanResult {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_order_no"
    )]
    pub out_order_no: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "state")]
    pub state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "state_description"
    )]
    pub state_description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "risk_fund")]
    pub risk_fund: Option<RiskFund>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "time_range"
    )]
    pub time_range: Option<TimeRange>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "location")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "notify_url"
    )]
    pub notify_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "order_id")]
    pub order_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "package")]
    pub package_x: Option<String>,
    #[serde(default, rename = "post_payments")]
    pub post_payments: Vec<PostPayment>,
    #[serde(default, rename = "post_discounts")]
    pub post_discounts: Vec<PostDiscount>,
    #[serde(default, rename = "need_collection")]
    pub need_collection: bool,
    #[serde(default, rename = "collection")]
    pub collection: serde_json::Value,
    #[serde(default, rename = "payScoreSignInfo")]
    pub pay_score_sign_info: std::collections::HashMap<Option<String>, Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub openid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "apply_permissions_token"
    )]
    pub apply_permissions_token: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_code"
    )]
    pub authorization_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_state"
    )]
    pub authorization_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_authorization_time"
    )]
    pub cancel_authorization_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "authorization_success_time"
    )]
    pub authorization_success_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "user_risk_level"
    )]
    pub user_risk_level: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "risk_level_version"
    )]
    pub risk_level_version: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_amount"
    )]
    pub total_amount: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "channel_id"
    )]
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_id")]
    pub plan_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_plan_no"
    )]
    pub merchant_plan_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_name")]
    pub plan_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_duration"
    )]
    pub plan_duration: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_state"
    )]
    pub plan_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_original_price"
    )]
    pub total_original_price: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deduction_quantity"
    )]
    pub deduction_quantity: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_actual_price"
    )]
    pub total_actual_price: Option<i32>,
    #[serde(default, rename = "plan_detail_list")]
    pub plan_detail_list: Vec<PayScorePlanDetailResult>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "stop_mchid"
    )]
    pub stop_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "stop_time")]
    pub stop_time: Option<String>,
}

impl WxPartnerPayScoreSignPlanResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxPartnerPayScoreSignPlanResult 解析失败: {e}"))
    }
}
