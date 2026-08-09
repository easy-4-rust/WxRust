//! 对应 Java `bean.card.AdvancedInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdvancedInfo {
    #[serde(rename = "use_condition", default)]
    pub use_condition: UseCondition,
    #[serde(rename = "abstract", default)]
    pub abstract_info: Abstract,
    #[serde(rename = "text_image_list", default)]
    pub text_image_list: Vec<TextImageList>,
    #[serde(rename = "business_service", default)]
    pub business_service_list: Vec<String>,
    #[serde(rename = "time_limit", default)]
    pub time_limits: Vec<TimeLimit>,
    #[serde(rename = "share_friends", default)]
    pub share_friends: bool,
}
