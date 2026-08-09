//! 对应 Java `me.chanjar.weixin.qidian.bean.dial.IVRListResponse.java`。

use serde::{Deserialize, Serialize};

use crate::bean::common::QidianResponse;
use crate::bean::dial::Ivr;

/// IVR 列表响应。
///
/// 对应 Java `IVRListResponse extends QidianResponse`：基类字段以
/// `#[serde(flatten)]` 展开（Rust 无继承，ADAPTED），`node` 为 IVR
/// 流程列表。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IVRListResponse {
    /// 响应基类字段（对应 Java 继承的 QidianResponse）
    #[serde(flatten)]
    pub base: QidianResponse,
    /// IVR 流程列表
    #[serde(default)]
    pub node: Option<Vec<Ivr>>,
}

impl IVRListResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("IVRListResponse 解析失败: {e}"))
    }
}
