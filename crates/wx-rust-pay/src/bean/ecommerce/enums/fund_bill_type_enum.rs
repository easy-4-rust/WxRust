//! 对应 Java `com.github.binarywang.wxpay.bean.ecommerce.enums.FundBillTypeEnum`。
//!
//! 由 `scripts/gen_pay_bean_enums.py` 生成：变体名即 Java 常量名
//! （serde 序列化值 = 常量名，对应 Java `name()`/`@SerializedName`），
//! 带构造参数的枚举生成与 Java 字段同名的 getter（对应 Lombok `@Getter`）。

#![allow(non_camel_case_types)]

/// FundBillTypeEnum（对应 Java `FundBillTypeEnum`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum FundBillTypeEnum {
    /// FUND_FLOW_BILL
    FUND_FLOW_BILL,
    /// SUB_FUND_FLOW_BILL
    SUB_FUND_FLOW_BILL,
}

impl FundBillTypeEnum {
    /// 获取 url（对应 Java `getUrl()`，Lombok @Getter）。
    pub fn url(&self) -> &'static str {
        match self {
            FundBillTypeEnum::FUND_FLOW_BILL => "%s/v3/bill/fundflowbill?%s",
            FundBillTypeEnum::SUB_FUND_FLOW_BILL => "%s/v3/ecommerce/bill/fundflowbill?%s",
        }
    }
}
