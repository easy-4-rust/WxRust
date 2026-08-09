//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.enums.BankAccountTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// BankAccountTypeEnum（对应 Java `BankAccountTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BankAccountTypeEnum {
    /// BANK_ACCOUNT_TYPE_CORPORATE
    BANK_ACCOUNT_TYPE_CORPORATE,
    /// BANK_ACCOUNT_TYPE_PERSONAL
    BANK_ACCOUNT_TYPE_PERSONAL,
}
