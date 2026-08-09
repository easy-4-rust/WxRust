//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpRecordSpStatus.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum WxCpRecordSpStatus {
    #[serde(rename = "1")]
    #[default]
    Auditing,
    #[serde(rename = "2")]
    Passed,
    #[serde(rename = "3")]
    Rejected,
    #[serde(rename = "4")]
    Turned,
    #[serde(rename = "11")]
    Withdrawn,
    #[serde(rename = "12")]
    Signed,
    #[serde(rename = "13")]
    Passedandsigned,
}
