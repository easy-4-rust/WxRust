//! 对应 Java `me.chanjar.weixin.open.bean.minishop.MinishopPicFile.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MinishopPicFile {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "payMediaId", default)]
    pub pay_media_id: String,
}
