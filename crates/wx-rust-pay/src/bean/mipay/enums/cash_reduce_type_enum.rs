//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.CashReduceTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// CashReduceTypeEnum（对应 Java `CashReduceTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum CashReduceTypeEnum {
    /// DEFAULT_REDUCE_TYPE
    DEFAULT_REDUCE_TYPE,
    /// HOSPITAL_REDUCE
    HOSPITAL_REDUCE,
    /// PHARMACY_DISCOUNT
    PHARMACY_DISCOUNT,
    /// DISCOUNT
    DISCOUNT,
    /// PRE_PAYMENT
    PRE_PAYMENT,
    /// DEPOSIT_DEDUCTION
    DEPOSIT_DEDUCTION,
}
