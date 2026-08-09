//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.MedInsPayStatusEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// MedInsPayStatusEnum（对应 Java `MedInsPayStatusEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MedInsPayStatusEnum {
    /// UNKNOWN_MED_INS_PAY_STATUS
    UNKNOWN_MED_INS_PAY_STATUS,
    /// MED_INS_PAY_CREATED
    MED_INS_PAY_CREATED,
    /// MED_INS_PAY_SUCCESS
    MED_INS_PAY_SUCCESS,
    /// MED_INS_PAY_REFUND
    MED_INS_PAY_REFUND,
    /// MED_INS_PAY_FAIL
    MED_INS_PAY_FAIL,
    /// NO_MED_INS_PAY
    NO_MED_INS_PAY,
}
