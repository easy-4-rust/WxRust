//! 对应 Java `me.chanjar.weixin.cp.bean.external.moment.MomentInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MomentInfo {
    #[serde(rename = "moment_id", default)]
    pub moment_id: String,
    #[serde(rename = "creator", default)]
    pub creator: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "create_type", default)]
    pub create_type: i32,
    #[serde(rename = "visible_type", default)]
    pub visible_type: i32,
    #[serde(rename = "text", default)]
    pub text: crate::bean::wx_cp_user_external_contact_info::Text,
    #[serde(rename = "image", default)]
    pub image: Vec<crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Image>,
    #[serde(rename = "video", default)]
    pub video: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Video,
    #[serde(rename = "link", default)]
    pub link: crate::bean::oa::doc::wx_cp_doc_sheet_data::Link,
    #[serde(rename = "location", default)]
    pub location: crate::bean::oa::applydata::content_value::Location,
}
