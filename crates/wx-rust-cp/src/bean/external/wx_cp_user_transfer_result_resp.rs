//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpUserTransferResultResp.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpUserTransferResultResp {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: String,
    #[serde(rename = "customer", default)]
    pub customer: Vec<TransferResult>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransferResult {
    #[serde(rename = "external_userid", default)]
    pub external_userid: String,
    #[serde(rename = "status", default)]
    pub status: Status,
    #[serde(rename = "takeover_time", default)]
    pub take_over_time: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum Status {
    #[serde(rename = "1")]
    #[default]
    Complete,
    #[serde(rename = "2")]
    Waiting,
    #[serde(rename = "3")]
    Refused,
    #[serde(rename = "4")]
    Limit,
}

impl WxCpUserTransferResultResp {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpUserTransferResultResp 解析失败: {e}"))
    }
}

impl WxCpUserTransferResultResp {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self)
            .map_err(|e| format!("WxCpUserTransferResultResp 序列化失败: {e}"))
    }
}
