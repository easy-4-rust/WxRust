//! 对应 Java `me.chanjar.weixin.cp.bean.export.WxCpExportRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExportRequest {
    #[serde(rename = "encoding_aeskey", default)]
    pub encoding_aes_key: String,
    #[serde(rename = "block_size", default)]
    pub block_size: i32,
    #[serde(rename = "tagid", default)]
    pub tag_id: i32,
}

impl WxCpExportRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpExportRequest 序列化失败: {e}"))
    }
}
