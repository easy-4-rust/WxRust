//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.enums.SpAccountTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// SpAccountTypeEnum（对应 Java `SpAccountTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SpAccountTypeEnum {
    /// BASIC
    BASIC,
    /// OPERATION
    OPERATION,
    /// FEES
    FEES,
}

impl SpAccountTypeEnum {
    /// 获取 value（对应 Java `getValue()`，Lombok @Getter）。
    pub fn value(&self) -> &'static str {
        match self {
            SpAccountTypeEnum::BASIC => "BASIC",
            SpAccountTypeEnum::OPERATION => "OPERATION",
            SpAccountTypeEnum::FEES => "FEES",
        }
    }
}
