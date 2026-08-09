//! 对应 Java `me.chanjar.weixin.qidian.bean.call.SwitchBoardList.java`。

use serde::{Deserialize, Serialize};

use crate::bean::call::SwitchBoard;

/// 总机号列表。
///
/// 对应 Java `SwitchBoardList`：`records` 为总机号记录，
/// `switch_boards()` 提取全部总机号字符串。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SwitchBoardList {
    /// 总机号记录
    #[serde(default)]
    pub records: Option<Vec<SwitchBoard>>,
}

impl SwitchBoardList {
    /// 提取全部总机号（对应 Java `switchBoards()`，基于 stream map）。
    pub fn switch_boards(&self) -> Vec<String> {
        self.records
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|s| s.switchboard.clone())
            .collect()
    }
}
