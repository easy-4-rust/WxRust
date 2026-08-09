//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudDatabaseCollectionGetResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudDatabaseCollectionGetResult {
    #[serde(rename = "pager", default)]
    pub pager: Pager,
    #[serde(rename = "collections", default)]
    pub collections: Vec<CollectionInfo>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CollectionInfo {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "count", default)]
    pub count: i64,
    #[serde(rename = "size", default)]
    pub size: i64,
    #[serde(rename = "index_count", default)]
    pub index_count: i64,
    #[serde(rename = "index_size", default)]
    pub index_size: i64,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pager {
    #[serde(rename = "Offset", default)]
    pub offset: i64,
    #[serde(rename = "Limit", default)]
    pub limit: i64,
    #[serde(rename = "Total", default)]
    pub total: i64,
}
