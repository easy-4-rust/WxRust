//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.PartnerUserSignPlanEntity.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartnerUserSignPlanEntity {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sign_plan_id"
    )]
    pub sign_plan_id: Option<String>,
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
        rename = "service_id"
    )]
    pub service_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mchid")]
    pub mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mchid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_appid: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_sign_plan_no"
    )]
    pub merchant_sign_plan_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_callback_url"
    )]
    pub merchant_callback_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_id")]
    pub plan_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "going_detail_no"
    )]
    pub going_detail_no: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sign_state"
    )]
    pub sign_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_sign_time"
    )]
    pub cancel_sign_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_sign_type"
    )]
    pub cancel_sign_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cancel_reason"
    )]
    pub cancel_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "plan_name")]
    pub plan_name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_over_time"
    )]
    pub plan_over_time: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_origin_price"
    )]
    pub total_origin_price: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "deduction_quantity"
    )]
    pub deduction_quantity: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "total_actual_price"
    )]
    pub total_actual_price: Option<i32>,
    #[serde(default, rename = "signed_detail_list")]
    pub signed_detail_list: Vec<PartnerUserSignPlanDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign_time")]
    pub sign_time: Option<String>,
}

impl PartnerUserSignPlanEntity {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("PartnerUserSignPlanEntity 解析失败: {e}"))
    }
}
