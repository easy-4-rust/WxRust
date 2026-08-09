//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpSpStatus.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum WxCpSpStatus {
    #[serde(rename = "1")]
    #[default]
    Auditing,
    #[serde(rename = "2")]
    Passed,
    #[serde(rename = "3")]
    Rejected,
    #[serde(rename = "4")]
    Undone,
    #[serde(rename = "6")]
    PassUndone,
    #[serde(rename = "7")]
    Deleted,
    #[serde(rename = "10")]
    AlreadyPay,
}
