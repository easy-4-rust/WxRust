//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpCorpConfInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpCorpConfInfo {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "lists", default)]
    pub lists: Vec<CorpConf>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CorpConf {
    #[serde(rename = "id", default)]
    pub id: i32,
    #[serde(rename = "name", default)]
    pub name: String,
    #[serde(rename = "time_attr", default)]
    pub time_attr: i32,
    #[serde(rename = "duration_type", default)]
    pub duration_type: i32,
    #[serde(rename = "quota_attr", default)]
    pub quota_attr: crate::bean::oa::wx_cp_corp_conf_info::QuotaAttr,
    #[serde(rename = "perday_duration", default)]
    pub perday_duration: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QuotaAttr {
    #[serde(rename = "type", default)]
    pub r#type: i32,
    #[serde(rename = "autoreset_time", default)]
    pub autoreset_time: i32,
    #[serde(rename = "autoreset_duration", default)]
    pub autoreset_duration: i32,
}

impl WxCpCorpConfInfo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpCorpConfInfo 解析失败: {e}"))
    }
}

impl WxCpCorpConfInfo {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpCorpConfInfo 序列化失败: {e}"))
    }
}
