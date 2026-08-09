//! 对应 Java `cn.binarywang.wx.miniapp.bean.WxMaMediaAsyncCheckResult.java`。
//!
//! 由 `scripts/gen_miniapp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMaMediaAsyncCheckResult {
    #[serde(rename = "trace_id", default)]
    pub trace_id: String,
    #[serde(rename = "result", default)]
    pub result: ResultBean,
    #[serde(rename = "detail", default)]
    pub detail: Vec<DetailBean>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResultBean {
    #[serde(rename = "suggest", default)]
    pub suggest: String,
    #[serde(rename = "label", default)]
    pub label: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DetailBean {
    #[serde(rename = "strategy", default)]
    pub strategy: String,
    #[serde(rename = "errcode", default)]
    pub errcode: i32,
    #[serde(rename = "suggest", default)]
    pub suggest: String,
    #[serde(rename = "label", default)]
    pub label: String,
    #[serde(rename = "prob", default)]
    pub prob: i32,
}

impl WxMaMediaAsyncCheckResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMaMediaAsyncCheckResult 解析失败: {e}"))
    }
}

impl WxMaMediaAsyncCheckResult {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxMaMediaAsyncCheckResult 序列化失败: {e}"))
    }
}
