//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxOpenMaHistoryVersion.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaHistoryVersion {
    #[serde(rename = "app_version", default)]
    pub app_version: i32,
    #[serde(rename = "user_version", default)]
    pub user_version: String,
    #[serde(rename = "user_desc", default)]
    pub user_desc: String,
    #[serde(rename = "commit_time", default)]
    pub commit_time: i32,
}
