//! 对应 Java `me.chanjar.weixin.cp.bean.external.moment.SenderList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SenderList {
    #[serde(rename = "user_list", default)]
    pub user_list: Vec<String>,
    #[serde(rename = "department_list", default)]
    pub department_list: Vec<String>,
}
