//! 对应 Java `me.chanjar.weixin.cp.bean.kf.msg.WxCpKfMiniProgramMsg.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kf::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpKfMiniProgramMsg {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "thumb_media_id", default)]
    pub thumb_media_id: String,
    #[serde(rename = "pagepath", default)]
    pub page_path: String,
}
