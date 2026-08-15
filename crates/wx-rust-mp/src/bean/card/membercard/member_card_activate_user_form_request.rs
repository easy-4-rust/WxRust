//! 对应 Java `bean.card.membercard.MemberCardActivateUserFormRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberCardActivateUserFormRequest {
    #[serde(rename = "card_id", default)]
    pub card_id: String,
    #[serde(rename = "service_statement", default)]
    pub service_statement: serde_json::Value,
    #[serde(rename = "bind_old_card", default)]
    pub bind_old_card: serde_json::Value,
    #[serde(rename = "required_form", default)]
    pub required_form: MemberCardUserForm,
    #[serde(rename = "optional_form", default)]
    pub optional_form: MemberCardUserForm,
}
