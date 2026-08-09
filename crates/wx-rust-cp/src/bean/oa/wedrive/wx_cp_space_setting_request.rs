//! 对应 Java `me.chanjar.weixin.cp.bean.oa.wedrive.WxCpSpaceSettingRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpSpaceSettingRequest {
    #[serde(rename = "userid", default)]
    pub user_id: String,
    #[serde(rename = "spaceid", default)]
    pub space_id: String,
    #[serde(rename = "enable_watermark", default)]
    pub enable_watermark: bool,
    #[serde(rename = "add_member_only_admin", default)]
    pub add_member_only_admin: bool,
    #[serde(rename = "enable_share_url", default)]
    pub enable_share_url: bool,
    #[serde(rename = "share_url_no_approve", default)]
    pub share_url_no_approve: bool,
    #[serde(rename = "share_url_no_approve_default_auth", default)]
    pub share_url_no_approve_default_auth: i32,
}

impl WxCpSpaceSettingRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpSpaceSettingRequest 解析失败: {e}"))
    }
}

impl WxCpSpaceSettingRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpSpaceSettingRequest 序列化失败: {e}"))
    }
}
