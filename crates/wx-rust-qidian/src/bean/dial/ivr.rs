//! 对应 Java `me.chanjar.weixin.qidian.bean.dial.Ivr.java`。

use serde::{Deserialize, Serialize};

/// IVR 流程信息。
///
/// 对应 Java `Ivr`：字段名即 Java 字段名（`ivr_id`/`ivr_name`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Ivr {
    /// IVR 流程 id
    #[serde(default)]
    pub ivr_id: Option<String>,
    /// IVR 流程名称
    #[serde(default)]
    pub ivr_name: Option<String>,
}
