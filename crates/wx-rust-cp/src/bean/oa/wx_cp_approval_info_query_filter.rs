//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalInfoQueryFilter.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalInfoQueryFilter {
    #[serde(rename = "key", default)]
    pub key: Key,
    #[serde(rename = "value", default)]
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Key {
    #[serde(rename = "template_id")]
    #[default]
    TemplateId,
    #[serde(rename = "creator")]
    Creator,
    #[serde(rename = "department")]
    Department,
    #[serde(rename = "sp_status")]
    SpStatus,
}

impl WxCpApprovalInfoQueryFilter {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpApprovalInfoQueryFilter 序列化失败: {e}"))
    }
}
