//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.SelfPayStatusEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// SelfPayStatusEnum（对应 Java `SelfPayStatusEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SelfPayStatusEnum {
    /// UNKNOWN_SELF_PAY_STATUS
    UNKNOWN_SELF_PAY_STATUS,
    /// SELF_PAY_CREATED
    SELF_PAY_CREATED,
    /// SELF_PAY_SUCCESS
    SELF_PAY_SUCCESS,
    /// SELF_PAY_REFUND
    SELF_PAY_REFUND,
    /// SELF_PAY_FAIL
    SELF_PAY_FAIL,
    /// NO_SELF_PAY
    NO_SELF_PAY,
}
