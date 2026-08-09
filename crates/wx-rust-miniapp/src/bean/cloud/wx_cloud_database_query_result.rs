//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudDatabaseQueryResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudDatabaseQueryResult {
    #[serde(rename = "pager", default)]
    pub pager: Pager,
    #[serde(rename = "data", default)]
    pub data: Vec<String>,
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
