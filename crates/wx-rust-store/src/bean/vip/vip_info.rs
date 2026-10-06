//! 对应 Java `com.binarywang.wxjava.store.bean.vip.VipInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{UserGradeInfo, UserInfo};

/// 微信小店 VipInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.vip.VipInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VipInfo {
    #[serde(rename = "openid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(rename = "union_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(rename = "user_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_info: Option<UserInfo>,
    #[serde(rename = "user_grade_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_grade_info: Option<UserGradeInfo>,
}
