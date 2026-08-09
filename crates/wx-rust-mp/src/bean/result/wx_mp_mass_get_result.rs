//! 对应 Java `bean.result.WxMpMassGetResult`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpMassGetResult {
    #[serde(rename = "msg_id", default)]
    pub msg_id: i64,
    #[serde(rename = "msg_status", default)]
    pub msgstatus: String,
}

impl WxMpMassGetResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpMassGetResult 解析失败: {e}"))
    }
}
