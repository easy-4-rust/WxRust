//! 对应 Java `me.chanjar.weixin.qidian.bean.call.GetSwitchBoardListResponse.java`。

use serde::{Deserialize, Serialize};

use crate::bean::call::SwitchBoardList;
use crate::bean::common::QidianResponse;

/// 总机号列表响应。
///
/// 对应 Java `GetSwitchBoardListResponse extends QidianResponse`：基类
/// 字段以 `#[serde(flatten)]` 展开（Rust 无继承，ADAPTED），`data` 为
/// 总机号列表。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GetSwitchBoardListResponse {
    /// 响应基类字段（对应 Java 继承的 QidianResponse）
    #[serde(flatten)]
    pub base: QidianResponse,
    /// 总机号列表
    #[serde(default)]
    pub data: Option<SwitchBoardList>,
}

impl GetSwitchBoardListResponse {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("GetSwitchBoardListResponse 解析失败: {e}"))
    }
}
