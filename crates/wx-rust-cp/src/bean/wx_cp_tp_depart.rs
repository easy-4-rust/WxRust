//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpDepart.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpDepart {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "enName", default)]
    pub en_name: String,
    #[serde(rename = "parentid", default)]
    pub parentid: i32,
    #[serde(rename = "order", default)]
    pub order: i32,
}

impl WxCpTpDepart {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpDepart 解析失败: {e}"))
    }
}

impl WxCpTpDepart {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpDepart 序列化失败: {e}"))
    }
}
