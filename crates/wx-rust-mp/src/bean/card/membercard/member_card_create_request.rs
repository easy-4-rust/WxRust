//! 对应 Java `bean.card.membercard.MemberCardCreateRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MemberCardCreateRequest {
    #[serde(rename = "card_type", default)]
    pub card_type: String,
    #[serde(rename = "member_card", default)]
    pub member_card: MemberCard,
}
