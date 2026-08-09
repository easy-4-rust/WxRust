//! 对应 Java `cn.binarywang.wx.miniapp.bean.xpay.WxMaXPayTeamInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaXPayTeamInfo {
    #[serde(rename = "ActivityId", default)]
    pub activity_id: String,
    #[serde(rename = "TeamId", default)]
    pub team_id: String,
    #[serde(rename = "TeamType", default)]
    pub team_type: i32,
    #[serde(rename = "TeamAction", default)]
    pub team_action: i32,
}
