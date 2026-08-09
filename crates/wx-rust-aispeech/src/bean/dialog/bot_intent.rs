//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.BotIntent.java`。

use serde::{Deserialize, Serialize};

/// bot 意图定义。
///
/// 对应 Java `BotIntent`：`importBotJson(mode, data)` 中 `data` 列表元素，
/// 描述一个技能下的意图、开关状态与问答语料。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BotIntent {
    /// 技能名称
    #[serde(default)]
    pub skill: Option<String>,
    /// 意图名称
    #[serde(default)]
    pub intent: Option<String>,
    /// 是否停用
    #[serde(default)]
    pub disable: Option<bool>,
    /// 用户问题语料
    #[serde(default)]
    pub questions: Option<Vec<String>>,
    /// 机器人答案语料
    #[serde(default)]
    pub answers: Option<Vec<String>>,
}
