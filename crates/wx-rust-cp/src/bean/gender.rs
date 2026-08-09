//! 对应 Java `me.chanjar.weixin.cp.bean.Gender.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 枚举生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    #[serde(rename = "UNDEFINED")]
    #[default]
    Undefined,
    #[serde(rename = "MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    Female,
}

impl Gender {
    /// 从字符串码构建（对应 Java `Gender.fromCode(String)`：`"0"` 未定义、
    /// `"1"` 男、`"2"` 女；未知码返回 `None`）。
    pub fn from_code(code: &str) -> Option<Gender> {
        match code {
            "0" => Some(Gender::Undefined),
            "1" => Some(Gender::Male),
            "2" => Some(Gender::Female),
            _ => None,
        }
    }

    /// 取字符串码（对应 Java `Gender.getCode()`）。
    pub fn code(&self) -> &'static str {
        match self {
            Gender::Undefined => "0",
            Gender::Male => "1",
            Gender::Female => "2",
        }
    }
}
