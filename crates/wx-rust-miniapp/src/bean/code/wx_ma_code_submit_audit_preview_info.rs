//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditPreviewInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditPreviewInfo {
    #[serde(rename = "video_id_list", default)]
    pub video_id_list: Vec<String>,
    #[serde(rename = "pic_id_list", default)]
    pub pic_id_list: Vec<String>,
}
