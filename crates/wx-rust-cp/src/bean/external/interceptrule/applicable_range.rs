//! 对应 Java `me.chanjar.weixin.cp.bean.external.interceptrule.ApplicableRange.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplicableRange {
    #[serde(rename = "user_list", default)]
    pub user_list: Vec<String>,
    #[serde(rename = "department_list", default)]
    pub department_list: Vec<i32>,
}

impl ApplicableRange {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("ApplicableRange 解析失败: {e}"))
    }
}

impl ApplicableRange {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("ApplicableRange 序列化失败: {e}"))
    }
}
