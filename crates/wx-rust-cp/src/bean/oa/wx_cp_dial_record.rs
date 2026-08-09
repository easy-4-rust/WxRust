//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpDialRecord.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDialRecord {
    #[serde(rename = "call_time", default)]
    pub call_time: i64,
    #[serde(rename = "total_duration", default)]
    pub total_duration: i32,
    #[serde(rename = "call_type", default)]
    pub call_type: i32,
    #[serde(rename = "caller", default)]
    pub caller: Caller,
    #[serde(rename = "callee", default)]
    pub callee: Vec<Callee>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Caller {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "duration", default)]
    pub duration: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Callee {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "phone", default)]
    pub phone: String,
    #[serde(rename = "duration", default)]
    pub duration: i32,
}
