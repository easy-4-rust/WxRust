//! 对应 Java `me.chanjar.weixin.cp.bean.living.WxCpLivingShareInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpLivingShareInfo {
    #[serde(rename = "livingid", default)]
    pub livingid: String,
    #[serde(rename = "viewer_userid", default)]
    pub viewer_userid: String,
    #[serde(rename = "viewer_external_userid", default)]
    pub viewer_external_userid: String,
    #[serde(rename = "invitor_userid", default)]
    pub invitor_userid: String,
    #[serde(rename = "invitor_external_userid", default)]
    pub invitor_external_userid: String,
}

impl WxCpLivingShareInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpLivingShareInfo 解析失败: {e}"))
    }
}

impl WxCpLivingShareInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpLivingShareInfo 序列化失败: {e}"))
    }
}
