//! 对应 Java `com.github.binarywang.wxpay.bean.payscore.UserSignPlanDetailMerchatNo.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UserSignPlanDetailMerchatNo {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "plan_detail_no"
    )]
    pub plan_detail_no: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "merchant_plan_detail_no"
    )]
    pub merchant_plan_detail_no: Option<String>,
}
