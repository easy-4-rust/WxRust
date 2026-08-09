//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocImageUploadResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocImageUploadResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "image_url", default)]
    pub image_url: String,
    #[serde(rename = "fileid", default)]
    pub file_id: String,
    #[serde(rename = "imageid", default)]
    pub image_id: String,
    #[serde(rename = "media_id", default)]
    pub media_id: String,
    #[serde(rename = "md5", default)]
    pub md5: String,
}

impl WxCpDocImageUploadResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocImageUploadResult 解析失败: {e}"))
    }
}

impl WxCpDocImageUploadResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpDocImageUploadResult 序列化失败: {e}"))
    }
}
