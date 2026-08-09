//! 对应 Java `bean.material.WxMpMaterialUploadResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMaterialUploadResult {
    #[serde(rename = "mediaId", default)]
    pub media_id: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "errCode", default)]
    pub err_code: i32,
    #[serde(rename = "errMsg", default)]
    pub err_msg: String,
}

impl WxMpMaterialUploadResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpMaterialUploadResult 解析失败: {e}"))
    }
}
