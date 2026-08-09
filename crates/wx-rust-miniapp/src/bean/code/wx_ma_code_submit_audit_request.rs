//! 对应 Java `cn.binarywang.wx.miniapp.bean.code.WxMaCodeSubmitAuditRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCodeSubmitAuditRequest {
    #[serde(rename = "item_list", default)]
    pub item_list: Vec<WxMaCodeSubmitAuditItem>,
    #[serde(rename = "feedback_info", default)]
    pub feedback_info: String,
    #[serde(rename = "feedback_stuff", default)]
    pub feedback_stuff: String,
    #[serde(rename = "preview_info", default)]
    pub preview_info: WxMaCodeSubmitAuditPreviewInfo,
    #[serde(rename = "version_desc", default)]
    pub version_desc: String,
    #[serde(rename = "ugc_declare", default)]
    pub ugc_declare: WxMaCodeSubmitAuditUgcDeclare,
    #[serde(rename = "privacy_api_not_use", default)]
    pub privacy_api_not_use: bool,
    #[serde(rename = "order_path", default)]
    pub order_path: String,
}

impl WxMaCodeSubmitAuditRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaCodeSubmitAuditRequest 序列化失败: {e}"))
    }
}
