//! 对应 Java `cn.binarywang.wx.miniapp.bean.marketing.WxMaUserAction.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaUserAction {
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
