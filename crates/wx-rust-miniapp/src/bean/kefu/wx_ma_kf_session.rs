//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu.WxMaKfSession.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaKfSession {
    #[serde(rename = "kf_account", default)]
    pub kf_account: String,
    #[serde(rename = "createtime", default)]
    pub create_time: i64,
}

impl WxMaKfSession {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaKfSession 解析失败: {e}"))
    }
}

impl WxMaKfSession {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaKfSession 序列化失败: {e}"))
    }
}
