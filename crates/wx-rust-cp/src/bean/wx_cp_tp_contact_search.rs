//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpContactSearch.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpContactSearch {
    #[serde(rename = "auth_corpid", default)]
    pub auth_corp_id: String,
    #[serde(rename = "query_word", default)]
    pub query_word: String,
    #[serde(rename = "query_type", default)]
    pub r#type: i32,
    #[serde(rename = "agentid", default)]
    pub agent_id: i32,
    #[serde(rename = "limit", default)]
    pub limit: i32,
    #[serde(rename = "full_match_field", default)]
    pub full_match_field: i32,
    #[serde(rename = "cursor", default)]
    pub cursor: String,
}

impl WxCpTpContactSearch {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpContactSearch 序列化失败: {e}"))
    }
}
