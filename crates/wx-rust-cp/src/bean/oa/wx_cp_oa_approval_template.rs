//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpOaApprovalTemplate.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpOaApprovalTemplate {
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "template_name", default)]
    pub template_name: Vec<crate::bean::oa::templatedata::template_title::TemplateTitle>,
    #[serde(rename = "template_content", default)]
    pub template_content: crate::bean::oa::wx_cp_oa_approval_template_result::TemplateContent,
}

impl WxCpOaApprovalTemplate {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpOaApprovalTemplate 解析失败: {e}"))
    }
}

impl WxCpOaApprovalTemplate {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpOaApprovalTemplate 序列化失败: {e}"))
    }
}
