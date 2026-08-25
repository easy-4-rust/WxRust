//! 点金计划操作结果。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.goldplan.GoldPlanResult`。

use serde::{Deserialize, Serialize};

/// 点金计划操作结果。
///
/// 对应 Java: `GoldPlanResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoldPlanResult {
    /// 特约商户号。
    ///
    /// 对应 Java: `GoldPlanResult#subMchId`
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mch_id: Option<String>,
}
