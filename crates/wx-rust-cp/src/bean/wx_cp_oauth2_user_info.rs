//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpOauth2UserInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOauth2UserInfo {
    #[serde(rename = "openId", default)]
    pub open_id: String,
    #[serde(rename = "deviceId", default)]
    pub device_id: String,
    #[serde(rename = "userId", default)]
    pub user_id: String,
    #[serde(rename = "userTicket", default)]
    pub user_ticket: String,
    #[serde(rename = "expiresIn", default)]
    pub expires_in: String,
    #[serde(rename = "externalUserId", default)]
    pub external_user_id: String,
    #[serde(rename = "parentUserId", default)]
    pub parent_user_id: String,
    #[serde(rename = "studentUserId", default)]
    pub student_user_id: String,
}
