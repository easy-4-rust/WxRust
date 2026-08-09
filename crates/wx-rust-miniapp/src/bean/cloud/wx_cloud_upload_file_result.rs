//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudUploadFileResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudUploadFileResult {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "token", default)]
    pub token: String,
    #[serde(rename = "authorization", default)]
    pub authorization: String,
    #[serde(rename = "file_id", default)]
    pub file_id: String,
    #[serde(rename = "cos_file_id", default)]
    pub cos_file_id: String,
}
