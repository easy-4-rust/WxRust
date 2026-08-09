//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenCommitStandardExt.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenCommitStandardExt {
    #[serde(rename = "extAppid", default)]
    pub ext_app_id: String,
    #[serde(rename = "ext", default)]
    pub ext: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "window", default)]
    pub window: std::collections::HashMap<String, serde_json::Value>,
}
