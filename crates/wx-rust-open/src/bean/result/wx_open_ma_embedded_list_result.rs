//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaEmbeddedListResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::ma::WxOpenMaEmbedded;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaEmbeddedListResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "wxa_embedded_list", default)]
    pub embedded_list: Vec<WxOpenMaEmbedded>,
    #[serde(rename = "embedded_flag", default)]
    pub embedded_flag: i32,
}

impl WxOpenMaEmbeddedListResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxOpenMaEmbeddedListResult 序列化失败: {e}"))
    }
}
