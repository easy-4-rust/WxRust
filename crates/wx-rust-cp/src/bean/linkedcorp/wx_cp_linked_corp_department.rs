//! 对应 Java `me.chanjar.weixin.cp.bean.linkedcorp.WxCpLinkedCorpDepartment.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLinkedCorpDepartment {
    #[serde(rename = "department_id", default)]
    pub department_id: String,
    #[serde(rename = "department_name", default)]
    pub department_name: String,
    #[serde(rename = "parentid", default)]
    pub parent_id: String,
    #[serde(rename = "order", default)]
    pub order: i32,
}
