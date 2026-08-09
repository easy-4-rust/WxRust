//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.MixPayStatusEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// MixPayStatusEnum（对应 Java `MixPayStatusEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MixPayStatusEnum {
    /// UNKNOWN_MIX_PAY_STATUS
    UNKNOWN_MIX_PAY_STATUS,
    /// MIX_PAY_CREATED
    MIX_PAY_CREATED,
    /// MIX_PAY_SUCCESS
    MIX_PAY_SUCCESS,
    /// MIX_PAY_REFUND
    MIX_PAY_REFUND,
    /// MIX_PAY_FAIL
    MIX_PAY_FAIL,
}
