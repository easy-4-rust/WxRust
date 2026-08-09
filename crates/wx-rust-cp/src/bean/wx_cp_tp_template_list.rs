//! 对应 Java `me.chanjar.weixin.cp.bean.WxCpTpTemplateList.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpTpTemplateList {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "template_list", default)]
    pub template_list: Vec<Template>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Template {
    #[serde(rename = "template_id", default)]
    pub template_id: String,
    #[serde(rename = "template_type", default)]
    pub template_type: i32,
    #[serde(rename = "app_name", default)]
    pub app_name: String,
    #[serde(rename = "logo_url", default)]
    pub logo_url: String,
    #[serde(rename = "app_desc", default)]
    pub app_desc: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}

impl WxCpTpTemplateList {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpTpTemplateList 解析失败: {e}"))
    }
}

impl WxCpTpTemplateList {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpTpTemplateList 序列化失败: {e}"))
    }
}
