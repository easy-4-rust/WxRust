//! 对应 Java `com.github.binarywang.wxpay.bean.applyconfirm.enums.ApplySubjectStateEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// ApplySubjectStateEnum（对应 Java `ApplySubjectStateEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ApplySubjectStateEnum {
    /// APPLYMENT_STATE_WAITTING_FOR_AUDIT
    APPLYMENT_STATE_WAITTING_FOR_AUDIT,
    /// APPLYMENT_STATE_EDITTING
    APPLYMENT_STATE_EDITTING,
    /// APPLYMENT_STATE_WAITTING_FOR_CONFIRM_CONTACT
    APPLYMENT_STATE_WAITTING_FOR_CONFIRM_CONTACT,
    /// APPLYMENT_STATE_WAITTING_FOR_CONFIRM_LEGALPERSON
    APPLYMENT_STATE_WAITTING_FOR_CONFIRM_LEGALPERSON,
    /// APPLYMENT_STATE_PASSED
    APPLYMENT_STATE_PASSED,
    /// APPLYMENT_STATE_REJECTED
    APPLYMENT_STATE_REJECTED,
    /// APPLYMENT_STATE_FREEZED
    APPLYMENT_STATE_FREEZED,
    /// APPLYMENT_STATE_CANCELED
    APPLYMENT_STATE_CANCELED,
}
