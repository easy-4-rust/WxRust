//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.PublishProgress.java`。

use serde::{Deserialize, Serialize};

/// bot 发布进度。
///
/// 对应 Java `PublishProgress`：`getPublishProgress(env)` 的返回值，
/// 描述指定环境下的发布状态与进度。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PublishProgress {
    /// 结束时间（对应 Java `@SerializedName("end_time")`）
    #[serde(rename = "end_time", default)]
    pub end_time: Option<String>,
    /// 进度（0-100）
    #[serde(default)]
    pub progress: Option<i32>,
    /// 发布状态
    #[serde(default)]
    pub status: Option<i32>,
}
