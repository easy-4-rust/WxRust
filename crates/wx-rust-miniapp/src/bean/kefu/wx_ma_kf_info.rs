//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu.WxMaKfInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaKfInfo {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "kf_nick", default)]
    pub kf_nick: String,
    #[serde(rename = "kf_id", default)]
    pub kf_id: String,
    #[serde(rename = "kf_headimgurl", default)]
    pub kf_head_img_url: String,
    #[serde(rename = "kf_wx", default)]
    pub kf_wx: String,
    #[serde(rename = "invite_wx", default)]
    pub invite_wx: String,
    #[serde(rename = "invite_expire_time", default)]
    pub invite_expire_time: i64,
    #[serde(rename = "invite_status", default)]
    pub invite_status: String,
}

impl WxMaKfInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaKfInfo 解析失败: {e}"))
    }
}

impl WxMaKfInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaKfInfo 序列化失败: {e}"))
    }
}
