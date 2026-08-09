//! 对应 Java `me.chanjar.weixin.cp.bean.export.WxCpExportResult.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpExportResult {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "status", default)]
    pub status: i32,
    #[serde(rename = "data_list", default)]
    pub data_list: Vec<ExportData>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExportData {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "size", default)]
    pub size: i32,
    #[serde(rename = "md5", default)]
    pub md5: String,
}
