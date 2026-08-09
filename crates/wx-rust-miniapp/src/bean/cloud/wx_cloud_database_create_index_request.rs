//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudDatabaseCreateIndexRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudDatabaseCreateIndexRequest {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "unique", default)]
    pub unique: bool,
    #[serde(rename = "keys", default)]
    pub keys: Vec<IndexKey>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IndexKey {
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "direction", default)]
    pub direction: String,
}
