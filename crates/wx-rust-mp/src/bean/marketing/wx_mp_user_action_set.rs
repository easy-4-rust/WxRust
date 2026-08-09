//! 对应 Java `bean.marketing.WxMpUserActionSet`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpUserActionSet {
    #[serde(rename = "user_action_set_id", default)]
    pub user_action_set_id: i64,
    #[serde(rename = "description", default)]
    pub description: String,
    #[serde(rename = "activate_status", default)]
    pub activate_status: bool,
    #[serde(rename = "created_time", default)]
    pub created_time: String,
}
