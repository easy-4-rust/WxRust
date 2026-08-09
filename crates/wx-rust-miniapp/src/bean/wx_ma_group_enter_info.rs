//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaGroupEnterInfo.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaGroupEnterInfo {
    #[serde(rename = "opengid", default)]
    pub open_g_id: String,
    #[serde(rename = "open_single_roomid", default)]
    pub open_single_roomid: String,
    #[serde(rename = "group_openid", default)]
    pub group_openid: String,
    #[serde(rename = "chat_type", default)]
    pub chat_type: i32,
}

impl WxMaGroupEnterInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaGroupEnterInfo 解析失败: {e}"))
    }
}
