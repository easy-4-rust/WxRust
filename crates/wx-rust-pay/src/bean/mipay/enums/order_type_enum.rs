//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.OrderTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// OrderTypeEnum（对应 Java `OrderTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OrderTypeEnum {
    /// UNKNOWN_ORDER_TYPE
    UNKNOWN_ORDER_TYPE,
    /// REG_PAY
    REG_PAY,
    /// DIAG_PAY
    DIAG_PAY,
    /// COVID_EXAM_PAY
    COVID_EXAM_PAY,
    /// IN_HOSP_PAY
    IN_HOSP_PAY,
    /// PHARMACY_PAY
    PHARMACY_PAY,
    /// INSURANCE_PAY
    INSURANCE_PAY,
    /// INT_REG_PAY
    INT_REG_PAY,
    /// INT_RE_DIAG_PAY
    INT_RE_DIAG_PAY,
    /// INT_RX_PAY
    INT_RX_PAY,
    /// COVID_ANTIGEN_PAY
    COVID_ANTIGEN_PAY,
    /// MED_PAY
    MED_PAY,
}
