//! 对应 Java `me.chanjar.weixin.cp.bean.oa.doc.WxCpFormStatisticRequest.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::oa::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpFormStatisticRequest {
    #[serde(rename = "repeated_id", default)]
    pub repeated_id: String,
    #[serde(rename = "req_type", default)]
    pub req_type: i32,
    #[serde(rename = "start_time", default)]
    pub start_time: i64,
    #[serde(rename = "end_time", default)]
    pub end_time: i64,
    #[serde(rename = "limit", default)]
    pub limit: i64,
    #[serde(rename = "cursor", default)]
    pub cursor: i64,
}

impl WxCpFormStatisticRequest {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpFormStatisticRequest 解析失败: {e}"))
    }
}

impl WxCpFormStatisticRequest {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpFormStatisticRequest 序列化失败: {e}"))
    }
}
