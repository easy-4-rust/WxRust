//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu.request.WxMaKfAccountRequest.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::kefu::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaKfAccountRequest {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "kf_nick", default)]
    pub kf_nick: String,
    #[serde(rename = "kf_pwd", default)]
    pub kf_pwd: String,
}

impl WxMaKfAccountRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaKfAccountRequest 解析失败: {e}"))
    }
}

impl WxMaKfAccountRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaKfAccountRequest 序列化失败: {e}"))
    }
}
