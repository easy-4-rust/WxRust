//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaStableAccessTokenRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaStableAccessTokenRequest {
    #[serde(rename = "grant_type", default)]
    pub grant_type: String,
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "secret", default)]
    pub secret: String,
    #[serde(rename = "force_refresh", default)]
    pub force_refresh: bool,
}

impl WxMaStableAccessTokenRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaStableAccessTokenRequest 序列化失败: {e}"))
    }
}
