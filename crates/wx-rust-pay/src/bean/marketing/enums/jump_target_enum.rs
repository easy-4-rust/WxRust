//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.enums.JumpTargetEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// JumpTargetEnum（对应 Java `JumpTargetEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum JumpTargetEnum {
    /// PAYMENT_CODE
    PAYMENT_CODE,
    /// MINI_PROGRAM
    MINI_PROGRAM,
    /// DEFAULT_PAGE
    DEFAULT_PAGE,
}

impl JumpTargetEnum {
    /// 获取 value（对应 Java `getValue()`，Lombok @Getter）。
    pub fn value(&self) -> &'static str {
        match self {
            JumpTargetEnum::PAYMENT_CODE => "PAYMENT_CODE",
            JumpTargetEnum::MINI_PROGRAM => "MINI_PROGRAM",
            JumpTargetEnum::DEFAULT_PAGE => "DEFAULT_PAGE",
        }
    }
}
