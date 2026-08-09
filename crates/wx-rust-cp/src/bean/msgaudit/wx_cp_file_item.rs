//! 对应 Java `me.chanjar.weixin.cp.bean.msgaudit.WxCpFileItem.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFileItem {
    #[serde(rename = "filename", default)]
    pub file_name: String,
    #[serde(rename = "md5sum", default)]
    pub md5_sum: String,
    #[serde(rename = "sdkfileid", default)]
    pub sdk_file_id: String,
    #[serde(rename = "filesize", default)]
    pub file_size: String,
}

impl WxCpFileItem {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFileItem 解析失败: {e}"))
    }
}

impl WxCpFileItem {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFileItem 序列化失败: {e}"))
    }
}
