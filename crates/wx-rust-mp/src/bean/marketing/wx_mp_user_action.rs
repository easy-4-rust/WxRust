//! 对应 Java `bean.marketing.WxMpUserAction`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpUserAction {
    #[serde(rename = "userActionSetId", default)]
    pub user_action_set_id: i64,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "actionTime", default)]
    pub action_time: i32,
    #[serde(rename = "actionType", default)]
    pub action_type: String,
    #[serde(rename = "leadsType", default)]
    pub leads_type: String,
    #[serde(rename = "clickId", default)]
    pub click_id: String,
    #[serde(rename = "actionParam", default)]
    pub action_param: i32,
}
