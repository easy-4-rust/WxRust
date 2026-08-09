//! 对应 Java `me.chanjar.weixin.cp.bean.external.msg.Video.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::external::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Video {
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "thumb_media_id", default)]
    pub thumb_media_id: String,
}
