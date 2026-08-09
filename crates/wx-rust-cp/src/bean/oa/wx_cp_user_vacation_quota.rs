//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpUserVacationQuota.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserVacationQuota {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "lists", default)]
    pub lists: Vec<VacationQuota>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VacationQuota {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "assignduration", default)]
    pub assign_duration: i32,
    #[serde(rename = "usedduration", default)]
    pub used_duration: i32,
    #[serde(rename = "leftduration", default)]
    pub left_duration: i32,
    #[serde(rename = "vacationname", default)]
    pub vacation_name: String,
}

impl WxCpUserVacationQuota {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUserVacationQuota 解析失败: {e}"))
    }
}

impl WxCpUserVacationQuota {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpUserVacationQuota 序列化失败: {e}"))
    }
}
