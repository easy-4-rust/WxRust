//! 小商店注册/资质审核状态。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopAuditStatus`。
//! 字段无 `@SerializedName`，Gson 反射线格式为 Java 字段名（camelCase），
//! Rust 以 `#[serde(rename = ...)]` 原样保留。
//!
//! ADAPTED：`wx_error` 为 `wx_rust_common::error::WxError`（无 Default 派生，
//! Java 可空），以 `Option` 表达。

use wx_rust_common::error::WxError;

/// 小商店注册/资质审核状态。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopAuditStatus {
    /// 微信错误信息（对应 Java `wxError`）。
    #[serde(rename = "wxError", default)]
    pub wx_error: Option<WxError>,
    /// 注册状态 0:成功 1:已发送协议还未签约 2:未发送协议或协议已过期，需发送协议。
    #[serde(rename = "registerStatus", default)]
    pub register_status: i32,
    /// 商家信息状态, 具体含义查看状态枚举值。
    #[serde(rename = "merchantInfoStatus", default)]
    pub merchant_info_status: i32,
    /// 账户验证状态, 具体含义查看状态枚举值。
    #[serde(rename = "acctVerifyStatus", default)]
    pub acct_verify_status: i32,
    /// 基础信息状态, 具体含义查看状态枚举值。
    #[serde(rename = "basicInfoStatus", default)]
    pub basic_info_status: i32,
    /// 支付签约状态, 具体含义查看状态枚举值。
    #[serde(rename = "paySignStatus", default)]
    pub pay_sign_status: i32,
    /// 基础信息驳回原因。
    #[serde(rename = "auditRejectReason", default)]
    pub audit_reject_reason: String,
    /// 法人验证链接。
    #[serde(rename = "legalValidationUrl", default)]
    pub legal_validation_url: String,
    /// 参数名。
    #[serde(rename = "payAuditDetailParamName", default)]
    pub pay_audit_detail_param_name: String,
    /// 支付资质驳回原因。
    #[serde(rename = "payAuditDetailRejectReason", default)]
    pub pay_audit_detail_reject_reason: String,
    /// 注册的appid。
    #[serde(rename = "registeredAppId", default)]
    pub registered_app_id: String,
}
