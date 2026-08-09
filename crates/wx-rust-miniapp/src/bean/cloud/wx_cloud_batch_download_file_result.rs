//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudBatchDownloadFileResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudBatchDownloadFileResult {
    #[serde(rename = "file_list", default)]
    pub file_list: Vec<FileDownloadInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FileDownloadInfo {
    #[serde(rename = "fileid", default)]
    pub file_id: String,
    #[serde(rename = "download_url", default)]
    pub download_url: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
}
