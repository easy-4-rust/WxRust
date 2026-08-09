//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.enums.IdTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// IdTypeEnum（对应 Java `IdTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum IdTypeEnum {
    /// IDENTIFICATION_TYPE_IDCARD
    IDENTIFICATION_TYPE_IDCARD,
    /// IDENTIFICATION_TYPE_OVERSEA_PASSPORT
    IDENTIFICATION_TYPE_OVERSEA_PASSPORT,
    /// IDENTIFICATION_TYPE_HONGKONG_PASSPORT
    IDENTIFICATION_TYPE_HONGKONG_PASSPORT,
    /// IDENTIFICATION_TYPE_MACAO_PASSPORT
    IDENTIFICATION_TYPE_MACAO_PASSPORT,
    /// IDENTIFICATION_TYPE_TAIWAN_PASSPORT
    IDENTIFICATION_TYPE_TAIWAN_PASSPORT,
    /// IDENTIFICATION_TYPE_FOREIGN_RESIDENT
    IDENTIFICATION_TYPE_FOREIGN_RESIDENT,
    /// IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT
    IDENTIFICATION_TYPE_HONGKONG_MACAO_RESIDENT,
    /// IDENTIFICATION_TYPE_TAIWAN_RESIDENT
    IDENTIFICATION_TYPE_TAIWAN_RESIDENT,
}
