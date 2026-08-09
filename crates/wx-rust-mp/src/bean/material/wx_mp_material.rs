//! 对应 Java `bean.material.WxMpMaterial`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterial {
    #[serde(rename = "name", default)]
    pub name: String,
    /// 文件路径（对应 Java `File`，Rust 中以本地路径承载；不参与 JSON 序列化）。
    #[serde(skip)]
    pub file: Option<String>,
    #[serde(rename = "videoTitle", default)]
    pub video_title: String,
    #[serde(rename = "videoIntroduction", default)]
    pub video_introduction: String,
}
