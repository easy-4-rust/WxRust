//! 对应 Java `me.chanjar.weixin.cp.bean.external.product.Attachment.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "image", default)]
    pub image: crate::bean::intelligentrobot::wx_cp_intelligent_robot_message::Image,
}
