//! 对应 Java `me.chanjar.weixin.qidian.bean.call.SwitchBoard.java`。

use serde::{Deserialize, Serialize};

/// 总机号（座席）信息。
///
/// 对应 Java `SwitchBoard`：一个总机号的呼入/呼出状态与归属地信息。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SwitchBoard {
    /// 总机号
    #[serde(default)]
    pub switchboard: Option<String>,
    /// 创建时间
    #[serde(default)]
    pub create_time: Option<String>,
    /// 呼入状态
    #[serde(default)]
    pub callin_status: Option<bool>,
    /// 呼出状态
    #[serde(default)]
    pub callout_status: Option<bool>,
    /// 运营商名称
    #[serde(default)]
    pub sp_name: Option<String>,
    /// 城市名称
    #[serde(default)]
    pub city_name: Option<String>,
}
