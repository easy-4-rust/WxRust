//! 对应 Java `me.chanjar.weixin.aispeech.bean.knowledge.KnowledgeMoveProgress.java`。

use serde::{Deserialize, Serialize};

/// 知识迁移进度。
///
/// 对应 Java `KnowledgeMoveProgress`：`getMoveProgress(taskId)` 的返回值，
/// 描述知识迁移异步任务的执行进度与结果。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeMoveProgress {
    /// 任务 id（对应 Java `@SerializedName("task_id")`）
    #[serde(rename = "task_id", default)]
    pub task_id: Option<String>,
    /// 任务状态
    #[serde(default)]
    pub status: Option<String>,
    /// 进度（0.0-1.0）
    #[serde(default)]
    pub progress: Option<f64>,
    /// 总条数
    #[serde(default)]
    pub total: Option<i32>,
    /// 已处理条数
    #[serde(default)]
    pub processed: Option<i32>,
    /// 消息
    #[serde(default)]
    pub message: Option<String>,
    /// 错误信息
    #[serde(default)]
    pub error: Option<String>,
}
