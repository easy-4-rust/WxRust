//! 对应 Java `me.chanjar.weixin.channel.bean.vip.VipInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VipInfo {
    #[serde(rename = "openid", default)]
    pub open_id: String,
    #[serde(rename = "union_id", default)]
    pub union_id: String,
    #[serde(rename = "user_info", default)]
    pub user_info: UserInfo,
    #[serde(rename = "user_grade_info", default)]
    pub user_grade_info: UserGradeInfo,
}
