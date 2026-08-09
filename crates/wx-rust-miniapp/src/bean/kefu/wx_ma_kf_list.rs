//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu.WxMaKfList.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaKfList {
    #[serde(rename = "kf_list", default)]
    pub kf_list: Vec<WxMaKfInfo>,
}

impl WxMaKfList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaKfList 解析失败: {e}"))
    }
}

impl WxMaKfList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxMaKfList 序列化失败: {e}"))
    }
}
