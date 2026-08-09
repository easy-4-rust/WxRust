//! 对应 Java `cn.binarywang.wx.miniapp.bean.cloud.WxCloudDatabaseUpdateResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCloudDatabaseUpdateResult {
    #[serde(rename = "matched", default)]
    pub matched: i64,
    #[serde(rename = "modified", default)]
    pub modified: i64,
    #[serde(rename = "id", default)]
    pub id: String,
}
