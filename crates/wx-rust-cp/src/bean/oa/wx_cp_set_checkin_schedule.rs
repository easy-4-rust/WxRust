//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpSetCheckinSchedule.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSetCheckinSchedule {
    #[serde(rename = "groupid", default)]
    pub group_id: i32,
    #[serde(rename = "items", default)]
    pub items: Vec<Item>,
    #[serde(rename = "yearmonth", default)]
    pub yearmonth: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "day", default)]
    pub day: i32,
    #[serde(rename = "schedule_id", default)]
    pub schedule_id: i32,
}
