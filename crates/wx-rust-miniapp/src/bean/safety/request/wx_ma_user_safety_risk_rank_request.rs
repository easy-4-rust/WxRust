//! 对应 Java `cn.binarywang.wx.miniapp.bean.safety.request.WxMaUserSafetyRiskRankRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::safety::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaUserSafetyRiskRankRequest {
    #[serde(rename = "appid", default)]
    pub appid: String,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "scene", default)]
    pub scene: i32,
    #[serde(rename = "mobile_no", default)]
    pub mobile_no: String,
    #[serde(rename = "client_ip", default)]
    pub client_ip: String,
    #[serde(rename = "email_address", default)]
    pub email_address: String,
    #[serde(rename = "extended_info", default)]
    pub extended_info: String,
    #[serde(rename = "is_test", default)]
    pub is_test: bool,
}

impl WxMaUserSafetyRiskRankRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaUserSafetyRiskRankRequest 序列化失败: {e}"))
    }
}
