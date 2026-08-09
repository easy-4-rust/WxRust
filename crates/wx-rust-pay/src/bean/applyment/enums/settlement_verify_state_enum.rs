//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.enums.SettlementVerifyStateEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// SettlementVerifyStateEnum（对应 Java `SettlementVerifyStateEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SettlementVerifyStateEnum {
    /// AUDIT_SUCCESS
    AUDIT_SUCCESS,
    /// AUDITING
    AUDITING,
    /// AUDIT_FAIL
    AUDIT_FAIL,
}
