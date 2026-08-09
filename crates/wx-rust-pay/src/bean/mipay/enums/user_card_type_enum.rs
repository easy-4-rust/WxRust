//! 对应 Java `com.github.binarywang.wxpay.bean.mipay.enums.UserCardTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// UserCardTypeEnum（对应 Java `UserCardTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum UserCardTypeEnum {
    /// UNKNOWN_USER_CARD_TYPE
    UNKNOWN_USER_CARD_TYPE,
    /// ID_CARD
    ID_CARD,
    /// HOUSEHOLD_REGISTRATION
    HOUSEHOLD_REGISTRATION,
    /// FOREIGNER_PASSPORT
    FOREIGNER_PASSPORT,
    /// MAINLAND_TRAVEL_PERMIT_FOR_TW
    MAINLAND_TRAVEL_PERMIT_FOR_TW,
    /// MAINLAND_TRAVEL_PERMIT_FOR_MO
    MAINLAND_TRAVEL_PERMIT_FOR_MO,
    /// MAINLAND_TRAVEL_PERMIT_FOR_HK
    MAINLAND_TRAVEL_PERMIT_FOR_HK,
    /// FOREIGN_PERMANENT_RESIDENT
    FOREIGN_PERMANENT_RESIDENT,
}
