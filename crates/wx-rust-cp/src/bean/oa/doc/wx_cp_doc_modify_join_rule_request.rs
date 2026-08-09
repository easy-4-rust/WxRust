//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocModifyJoinRuleRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocModifyJoinRuleRequest {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "enable_corp_internal", default)]
    pub enable_corp_internal: bool,
    #[serde(rename = "corp_internal_auth", default)]
    pub corp_internal_auth: i32,
    #[serde(rename = "enable_corp_external", default)]
    pub enable_corp_external: bool,
    #[serde(rename = "corp_external_auth", default)]
    pub corp_external_auth: i32,
    #[serde(rename = "corp_internal_approve_only_by_admin", default)]
    pub corp_internal_approve_only_by_admin: bool,
    #[serde(rename = "corp_external_approve_only_by_admin", default)]
    pub corp_external_approve_only_by_admin: bool,
    #[serde(rename = "ban_share_external", default)]
    pub ban_share_external: bool,
    #[serde(rename = "update_co_auth_list", default)]
    pub update_co_auth_list: bool,
    #[serde(rename = "co_auth_list", default)]
    pub co_auth_list: Vec<crate::bean::oa::doc::wx_cp_doc_auth_info::CoAuthInfo>,
}

impl WxCpDocModifyJoinRuleRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json)
            .map_err(|e| format!("WxCpDocModifyJoinRuleRequest 解析失败: {e}"))
    }
}

impl WxCpDocModifyJoinRuleRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpDocModifyJoinRuleRequest 序列化失败: {e}"))
    }
}
