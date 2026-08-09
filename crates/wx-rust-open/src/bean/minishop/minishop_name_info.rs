//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopNameInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopNameInfo {
    #[serde(rename = "nickName", default)]
    pub nick_name: String,
    #[serde(rename = "abbr", default)]
    pub abbr: String,
    #[serde(rename = "introduction", default)]
    pub introduction: String,
    #[serde(rename = "namingOtherStuff", default)]
    pub naming_other_stuff: Vec<String>,
}
