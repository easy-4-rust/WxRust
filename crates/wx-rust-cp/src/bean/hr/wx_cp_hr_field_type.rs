//! 对应 Java `me.chanjar.weixin.cp.bean.hr.WxCpHrFieldType.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum WxCpHrFieldType {
    #[serde(rename = "TEXT")]
    #[default]
    Text,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "SINGLE_SELECT")]
    SingleSelect,
    #[serde(rename = "MULTI_SELECT")]
    MultiSelect,
    #[serde(rename = "ATTACHMENT")]
    Attachment,
    #[serde(rename = "PHONE")]
    Phone,
    #[serde(rename = "EMAIL")]
    Email,
}
