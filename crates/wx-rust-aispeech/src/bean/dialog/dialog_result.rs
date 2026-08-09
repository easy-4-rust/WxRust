//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.DialogResult.java`。

use serde::{Deserialize, Serialize};

/// 对话查询结果。
///
/// 对应 Java `DialogResult`：机器人回复内容、命中的技能/意图、候选选项与
/// 槽位信息。`raw_answer` 为 `answer` 形如 JSON 时解析出的原始结构
/// （对应 Java `JsonElement rawAnswer`，由 `query` 成功应答后写入）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DialogResult {
    /// 回复内容（可为 JSON 字符串）
    #[serde(default)]
    pub answer: Option<String>,
    /// 回复类型（对应 Java `@SerializedName("answer_type")`）
    #[serde(rename = "answer_type", default)]
    pub answer_type: Option<String>,
    /// 命中的技能名称（对应 Java `@SerializedName("skill_name")`）
    #[serde(rename = "skill_name", default)]
    pub skill_name: Option<String>,
    /// 命中的意图名称（对应 Java `@SerializedName("intent_name")`）
    #[serde(rename = "intent_name", default)]
    pub intent_name: Option<String>,
    /// 消息 id（对应 Java `@SerializedName("msg_id")`）
    #[serde(rename = "msg_id", default)]
    pub msg_id: Option<String>,
    /// 候选选项列表
    #[serde(default)]
    pub options: Option<Vec<DialogOption>>,
    /// 回复状态
    #[serde(default)]
    pub status: Option<String>,
    /// 槽位详情列表
    #[serde(default)]
    pub slots: Option<Vec<SlotDetail>>,
    /// 原始回复结构（对应 Java `JsonElement rawAnswer`；本地写入，不参与序列化）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_answer: Option<serde_json::Value>,
}

/// 候选选项（对应 Java 内部类 `DialogResult.Option`；
/// Rust 命名为 `DialogOption` 避免与 `std::option::Option` 冲突，ADAPTED）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DialogOption {
    /// 答案节点名称（对应 Java `@SerializedName("ans_node_name")`）
    #[serde(rename = "ans_node_name", default)]
    pub ans_node_name: Option<String>,
    /// 选项标题
    #[serde(default)]
    pub title: Option<String>,
    /// 选项内容
    #[serde(default)]
    pub answer: Option<String>,
    /// 置信度
    #[serde(default)]
    pub confidence: Option<f32>,
}

/// 槽位详情（对应 Java 内部类 `DialogResult.SlotDetail`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SlotDetail {
    /// 槽位名
    #[serde(default)]
    pub name: Option<String>,
    /// 槽位值
    #[serde(default)]
    pub value: Option<String>,
    /// 归一化值
    #[serde(default)]
    pub norm: Option<String>,
}
