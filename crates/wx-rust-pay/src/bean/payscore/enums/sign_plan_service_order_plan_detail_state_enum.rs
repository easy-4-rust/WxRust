//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.enums.SignPlanServiceOrderPlanDetailStateEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// SignPlanServiceOrderPlanDetailStateEnum（对应 Java `SignPlanServiceOrderPlanDetailStateEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SignPlanServiceOrderPlanDetailStateEnum {
    /// NOT_USED
    NOT_USED,
    /// USING
    USING,
    /// USED
    USED,
    /// SIGN_PLAN_DETAIL_CANCEL
    SIGN_PLAN_DETAIL_CANCEL,
}
