//! 对应 Java `me.chanjar.weixin.qidian.bean.dial.IVRDialResponse.java`。

use serde::{Deserialize, Serialize};

use crate::bean::common::QidianResponse;

/// IVR 外呼响应。
///
/// 对应 Java `IVRDialResponse extends QidianResponse`：基类字段以
/// `#[serde(flatten)]` 展开（Rust 无继承，ADAPTED），`callid` 为呼叫 id。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IVRDialResponse {
    /// 响应基类字段（对应 Java 继承的 QidianResponse）
    #[serde(flatten)]
    pub base: QidianResponse,
    /// 呼叫 id
    #[serde(default)]
    pub callid: Option<String>,
}

impl IVRDialResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("IVRDialResponse 解析失败: {e}"))
    }
}
