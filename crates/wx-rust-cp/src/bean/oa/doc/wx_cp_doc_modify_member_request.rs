//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpDocModifyMemberRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpDocModifyMemberRequest {
    #[serde(rename = "docid", default)]
    pub doc_id: String,
    #[serde(rename = "update_file_member_list", default)]
    pub update_file_member_list: Vec<crate::bean::oa::doc::wx_cp_doc_auth_info::DocMember>,
    #[serde(rename = "del_file_member_list", default)]
    pub del_file_member_list: Vec<crate::bean::oa::doc::wx_cp_doc_auth_info::DocMember>,
}

impl WxCpDocModifyMemberRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpDocModifyMemberRequest 解析失败: {e}"))
    }
}

impl WxCpDocModifyMemberRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpDocModifyMemberRequest 序列化失败: {e}"))
    }
}
