//! 对应 Java `bean.card.membercard.MemberCardUserForm`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
use crate::bean::card::enums::*;
use crate::bean::card::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberCardUserForm {
    #[serde(rename = "can_modify", default)]
    pub can_modify: bool,
    #[serde(rename = "custom_field_list", default)]
    pub custom_field_list: Vec<String>,
    #[serde(rename = "common_field_id_list", default)]
    pub wechat_field_id_list: Vec<String>,
}
