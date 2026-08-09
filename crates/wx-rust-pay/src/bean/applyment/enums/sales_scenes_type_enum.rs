//! 对应 Java `com.github.binarywang.wxpay.bean.applyment.enums.SalesScenesTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// SalesScenesTypeEnum（对应 Java `SalesScenesTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SalesScenesTypeEnum {
    /// SALES_SCENES_STORE
    SALES_SCENES_STORE,
    /// SALES_SCENES_MP
    SALES_SCENES_MP,
    /// SALES_SCENES_MINI_PROGRAM
    SALES_SCENES_MINI_PROGRAM,
    /// SALES_SCENES_WEB
    SALES_SCENES_WEB,
    /// SALES_SCENES_APP
    SALES_SCENES_APP,
    /// SALES_SCENES_WEWORK
    SALES_SCENES_WEWORK,
}
