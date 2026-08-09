//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.enums.StockTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// StockTypeEnum（对应 Java `StockTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum StockTypeEnum {
    /// NORMAL
    NORMAL,
    /// DISCOUNT
    DISCOUNT,
    /// EXCHANGE
    EXCHANGE,
}

impl StockTypeEnum {
    /// 获取 value（对应 Java `getValue()`，Lombok @Getter）。
    pub fn value(&self) -> &'static str {
        match self {
            StockTypeEnum::NORMAL => "NORMAL",
            StockTypeEnum::DISCOUNT => "DISCOUNT",
            StockTypeEnum::EXCHANGE => "EXCHANGE",
        }
    }
}
