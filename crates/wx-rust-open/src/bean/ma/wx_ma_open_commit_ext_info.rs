//! 对应 Java `me.chanjar.weixin.open.bean.ma.WxMaOpenCommitExtInfo.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaOpenCommitExtInfo {
    #[serde(rename = "extAppid", default)]
    pub ext_appid: String,
    #[serde(rename = "extEnable", default)]
    pub ext_enable: bool,
    #[serde(rename = "directCommit", default)]
    pub direct_commit: bool,
    #[serde(rename = "ext", default)]
    pub ext_map: std::collections::HashMap<String, serde_json::Value>,
    #[serde(rename = "extPages", default)]
    pub ext_pages: std::collections::HashMap<String, WxMaOpenPage>,
    #[serde(rename = "pages", default)]
    pub page_list: Vec<String>,
    #[serde(rename = "subpackages", default)]
    pub subpackage_list: Vec<WxMaOpenSubpackage>,
    #[serde(rename = "window", default)]
    pub window: WxMaOpenWindow,
    #[serde(rename = "networkTimeout", default)]
    pub network_timeout: WxMaOpenNetworkTimeout,
    #[serde(rename = "tabBar", default)]
    pub tab_bar: WxMaOpenTabBar,
    #[serde(rename = "requiredPrivateInfos", default)]
    pub required_private_infos: Vec<String>,
}

impl WxMaOpenCommitExtInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaOpenCommitExtInfo 序列化失败: {e}"))
    }
}
