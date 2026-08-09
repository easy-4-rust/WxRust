//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpProviderToken.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpProviderToken {
    #[serde(rename = "provider_access_token", default)]
    pub provider_access_token: String,
    #[serde(rename = "expires_in", default)]
    pub expires_in: i32,
}

impl WxCpProviderToken {
    /// 构建服务商凭证。
    pub fn new(provider_access_token: Option<String>, expires_in: Option<i32>) -> Self {
        Self {
            provider_access_token: provider_access_token.unwrap_or_default(),
            expires_in: expires_in.unwrap_or_default(),
        }
    }
}

impl WxCpProviderToken {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpProviderToken 解析失败: {e}"))
    }
}
