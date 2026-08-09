//! 对应 Java `bean.guide.WxMpGuideCardMaterialInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpGuideCardMaterialInfo {
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "path", default)]
    pub path: String,
    #[serde(rename = "picurl", default)]
    pub pic_url: String,
    #[serde(rename = "master_id", default)]
    pub master_id: i64,
    #[serde(rename = "slave_id", default)]
    pub slave_id: i64,
}
