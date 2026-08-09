//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpAddMomentTask.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpAddMomentTask {
    #[serde(rename = "visible_range", default)]
    pub visible_range: crate::bean::external::moment::visible_range::VisibleRange,
    #[serde(rename = "text", default)]
    pub text: crate::bean::wx_cp_user_external_contact_info::Text,
    #[serde(rename = "attachments", default)]
    pub attachments: Vec<crate::bean::oa::mail::wx_cp_mail_common_send_request::Attachment>,
}

impl WxCpAddMomentTask {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpAddMomentTask 序列化失败: {e}"))
    }
}
