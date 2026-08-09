//! 对应 Java `me.chanjar.weixin.cp.bean.oa.templatedata.control.TemplateDate.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::templatedata::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TemplateDate {
    #[serde(rename = "type", default)]
    pub r#type: String,
}
