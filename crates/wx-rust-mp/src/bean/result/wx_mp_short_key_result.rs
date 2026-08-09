//! 短 key 解析结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpShortKeyResult`。

use serde::{Deserialize, Serialize};

/// 短 key 解析结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpShortKeyResult {
    /// 长信息。
    #[serde(rename = "long_data", default)]
    pub long_data: String,
    /// 创建的时间戳。
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    /// 剩余的过期秒数。
    #[serde(rename = "expire_seconds", default)]
    pub expire_seconds: i64,
}

impl WxMpShortKeyResult {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("短 key 解析结果解析失败: {e}"))
    }
}
