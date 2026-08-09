//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxOpenMaEmbedded.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaEmbedded {
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "headimg", default)]
    pub head_img: String,
    #[serde(rename = "nickname", default)]
    pub nick_name: String,
    #[serde(rename = "reason", default)]
    pub reason: String,
    #[serde(rename = "status", default)]
    pub status: String,
}

impl WxOpenMaEmbedded {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxOpenMaEmbedded 序列化失败: {e}"))
    }
}
