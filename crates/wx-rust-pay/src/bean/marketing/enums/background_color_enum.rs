//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.enums.BackgroundColorEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// BackgroundColorEnum（对应 Java `BackgroundColorEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BackgroundColorEnum {
    /// COLOR010
    COLOR010,
    /// COLOR020
    COLOR020,
    /// COLOR030
    COLOR030,
    /// COLOR040
    COLOR040,
    /// COLOR050
    COLOR050,
    /// COLOR060
    COLOR060,
    /// COLOR070
    COLOR070,
    /// COLOR080
    COLOR080,
    /// COLOR081
    COLOR081,
    /// COLOR082
    COLOR082,
    /// COLOR090
    COLOR090,
    /// COLOR100
    COLOR100,
    /// COLOR101
    COLOR101,
    /// COLOR102
    COLOR102,
}

impl BackgroundColorEnum {
    /// 获取 value（对应 Java `getValue()`，Lombok @Getter）。
    pub fn value(&self) -> &'static str {
        match self {
            BackgroundColorEnum::COLOR010 => "COLOR010",
            BackgroundColorEnum::COLOR020 => "COLOR020",
            BackgroundColorEnum::COLOR030 => "COLOR030",
            BackgroundColorEnum::COLOR040 => "COLOR040",
            BackgroundColorEnum::COLOR050 => "COLOR050",
            BackgroundColorEnum::COLOR060 => "COLOR060",
            BackgroundColorEnum::COLOR070 => "COLOR070",
            BackgroundColorEnum::COLOR080 => "COLOR080",
            BackgroundColorEnum::COLOR081 => "COLOR081",
            BackgroundColorEnum::COLOR082 => "COLOR082",
            BackgroundColorEnum::COLOR090 => "COLOR090",
            BackgroundColorEnum::COLOR100 => "COLOR100",
            BackgroundColorEnum::COLOR101 => "COLOR101",
            BackgroundColorEnum::COLOR102 => "COLOR102",
        }
    }
    /// 获取 code（对应 Java `getCode()`，Lombok @Getter）。
    pub fn code(&self) -> &'static str {
        match self {
            BackgroundColorEnum::COLOR010 => "#63B359",
            BackgroundColorEnum::COLOR020 => "#2C9F67",
            BackgroundColorEnum::COLOR030 => "#509FC9",
            BackgroundColorEnum::COLOR040 => "#5885CF",
            BackgroundColorEnum::COLOR050 => "#9062C0",
            BackgroundColorEnum::COLOR060 => "#D09A45",
            BackgroundColorEnum::COLOR070 => "#E4B138",
            BackgroundColorEnum::COLOR080 => "#EE903C",
            BackgroundColorEnum::COLOR081 => "#F08500",
            BackgroundColorEnum::COLOR082 => "#A9D92D",
            BackgroundColorEnum::COLOR090 => "#DD6549",
            BackgroundColorEnum::COLOR100 => "#CC463D",
            BackgroundColorEnum::COLOR101 => "#CF3E36",
            BackgroundColorEnum::COLOR102 => "#5E6671",
        }
    }
}
