//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaCode2VerifyInfoResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaCode2VerifyInfoResult {
    #[serde(rename = "session_key", default)]
    pub session_key: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "unionid", default)]
    pub unionid: String,
    #[serde(rename = "is_limit", default)]
    pub is_limit: bool,
}

impl WxMaCode2VerifyInfoResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaCode2VerifyInfoResult 解析失败: {e}"))
    }
}
