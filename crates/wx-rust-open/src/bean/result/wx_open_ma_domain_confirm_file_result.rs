//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaDomainConfirmFileResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaDomainConfirmFileResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "file_name", default)]
    pub file_name: String,
    #[serde(rename = "file_content", default)]
    pub file_content: String,
}

impl WxOpenMaDomainConfirmFileResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxOpenMaDomainConfirmFileResult 解析失败: {e}"))
    }
}
