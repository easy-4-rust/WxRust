//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpCorp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpCorp {
    #[serde(rename = "corpid", default)]
    pub corp_id: String,
    #[serde(rename = "corp_name", default)]
    pub corp_name: String,
    #[serde(rename = "corp_full_name", default)]
    pub corp_full_name: String,
    #[serde(rename = "corp_type", default)]
    pub corp_type: String,
    #[serde(rename = "corp_square_logo_url", default)]
    pub corp_square_logo_url: String,
    #[serde(rename = "corp_user_max", default)]
    pub corp_user_max: String,
    #[serde(rename = "permanent_code", default)]
    pub permanent_code: String,
    #[serde(rename = "auth_info", default)]
    pub auth_info: String,
}

impl WxCpTpCorp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpCorp 解析失败: {e}"))
    }
}

impl WxCpTpCorp {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpCorp 序列化失败: {e}"))
    }
}
