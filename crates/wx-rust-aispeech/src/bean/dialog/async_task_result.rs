//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.AsyncTaskResult.java`。

use serde::{Deserialize, Serialize};

/// 异步任务结果。
///
/// 对应 Java `AsyncTaskResult`：bot 导入等异步任务的进度与结果。
/// `success_skill_info` 为任意 JSON（Java `JsonElement`），
/// `success_skill_info_list` 为结构化技能信息列表。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AsyncTaskResult {
    /// 任务状态
    #[serde(default)]
    pub state: Option<i32>,
    /// 状态描述
    #[serde(default)]
    pub msg: Option<String>,
    /// 进度（0-100）
    #[serde(default)]
    pub progress: Option<i32>,
    /// 开始时间（时间戳）
    #[serde(default)]
    pub start: Option<i64>,
    /// 结束时间（时间戳）
    #[serde(default)]
    pub end: Option<i64>,
    /// 结果下载地址
    #[serde(default)]
    pub url: Option<String>,
    /// 总条数（对应 Java `totalCount`；Gson 无 `@SerializedName`，
    /// JSON 键为 camelCase `totalCount`）
    #[serde(rename = "totalCount", default)]
    pub total_count: Option<i32>,
    /// 成功条数（对应 Java `successCount`；JSON 键为 camelCase）
    #[serde(rename = "successCount", default)]
    pub success_count: Option<i32>,
    /// 失败条数（对应 Java `failCount`；JSON 键为 camelCase）
    #[serde(rename = "failCount", default)]
    pub fail_count: Option<i32>,
    /// 成功技能信息（原始 JSON，对应 Java `JsonElement successSkillInfo`；
    /// JSON 键为 camelCase）
    #[serde(rename = "successSkillInfo", default)]
    pub success_skill_info: Option<serde_json::Value>,
    /// 成功技能信息列表（对应 Java `successSkillInfoList`；JSON 键为
    /// camelCase）
    #[serde(rename = "successSkillInfoList", default)]
    pub success_skill_info_list: Option<Vec<SkillInfo>>,
}

/// 技能信息（对应 Java 内部类 `AsyncTaskResult.SkillInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SkillInfo {
    /// 技能 id
    #[serde(default)]
    pub id: Option<i64>,
    /// 技能名称
    #[serde(default)]
    pub name: Option<String>,
    /// 意图列表
    #[serde(default)]
    pub intents: Option<Vec<IntentInfo>>,
}

/// 意图信息（对应 Java 内部类 `AsyncTaskResult.IntentInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct IntentInfo {
    /// 意图 id
    #[serde(default)]
    pub id: Option<i64>,
    /// 意图名称
    #[serde(default)]
    pub name: Option<String>,
}
