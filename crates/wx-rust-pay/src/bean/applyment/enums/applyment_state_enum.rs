//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.enums.ApplymentStateEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// ApplymentStateEnum（对应 Java `ApplymentStateEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ApplymentStateEnum {
    /// APPLYMENT_STATE_EDITTING
    APPLYMENT_STATE_EDITTING,
    /// APPLYMENT_STATE_AUDITING
    APPLYMENT_STATE_AUDITING,
    /// APPLYMENT_STATE_REJECTED
    APPLYMENT_STATE_REJECTED,
    /// APPLYMENT_STATE_TO_BE_CONFIRMED
    APPLYMENT_STATE_TO_BE_CONFIRMED,
    /// APPLYMENT_STATE_TO_BE_SIGNED
    APPLYMENT_STATE_TO_BE_SIGNED,
    /// APPLYMENT_STATE_SIGNING
    APPLYMENT_STATE_SIGNING,
    /// APPLYMENT_STATE_FINISHED
    APPLYMENT_STATE_FINISHED,
    /// APPLYMENT_STATE_CANCELED
    APPLYMENT_STATE_CANCELED,
}
