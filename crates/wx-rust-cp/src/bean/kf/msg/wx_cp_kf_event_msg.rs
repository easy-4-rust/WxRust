//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfEventMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfEventMsg {
    #[serde(rename = "event_type", default)]
    pub event_type: String,
    #[serde(rename = "open_kfid", default)]
    pub open_kfid: String,
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "servicer_userid", default)]
    pub servicer_user_id: String,
    #[serde(rename = "old_servicer_userid", default)]
    pub old_servicer_user_id: String,
    #[serde(rename = "new_servicer_userid", default)]
    pub new_servicer_user_id: String,
    #[serde(rename = "scene", default)]
    pub scene: String,
    #[serde(rename = "scene_param", default)]
    pub scene_param: String,
    #[serde(rename = "welcome_code", default)]
    pub welcome_code: String,
    #[serde(rename = "fail_msgid", default)]
    pub fail_msg_id: String,
    #[serde(rename = "fail_type", default)]
    pub fail_type: i32,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "change_type", default)]
    pub change_type: i32,
    #[serde(rename = "msg_code", default)]
    pub msg_code: String,
    #[serde(rename = "recall_msgid", default)]
    pub recall_msg_id: String,
}
