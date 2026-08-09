//! 对应 Java `bean.tag.WxTagListUser`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxTagListUser {
    #[serde(rename = "count", default)]
    pub count: i32,
    #[serde(rename = "data", default)]
    pub data: WxTagListUserData,
    #[serde(rename = "next_openid", default)]
    pub next_openid: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxTagListUserData {
    #[serde(rename = "openid", default)]
    pub openid_list: Vec<String>,
}

impl WxTagListUser {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxTagListUser 解析失败: {e}"))
    }
}
