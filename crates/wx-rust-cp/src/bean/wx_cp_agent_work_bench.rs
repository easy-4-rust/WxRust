//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpAgentWorkBench.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAgentWorkBench {
    #[serde(rename = "type", default)]
    pub r#type: String,
    #[serde(rename = "userId", default)]
    pub user_id: String,
    #[serde(rename = "useridList", default)]
    pub userid_list: Vec<String>,
    #[serde(rename = "agentId", default)]
    pub agent_id: i64,
    #[serde(rename = "jumpUrl", default)]
    pub jump_url: String,
    #[serde(rename = "pagePath", default)]
    pub page_path: String,
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "replaceUserData", default)]
    pub replace_user_data: bool,
    #[serde(rename = "enableWebviewClick", default)]
    pub enable_webview_click: bool,
    #[serde(rename = "height", default)]
    pub height: String,
    #[serde(rename = "hideTitle", default)]
    pub hide_title: bool,
    #[serde(rename = "keyDataList", default)]
    pub key_data_list: Vec<crate::bean::workbench::work_bench_key_data::WorkBenchKeyData>,
    #[serde(rename = "lists", default)]
    pub lists: Vec<crate::bean::workbench::work_bench_list::WorkBenchList>,
}
