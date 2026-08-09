//! 智能对话数据对象。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.bean` 包：`dialog`（对话机器人）与
//! `knowledge`（知识库助理）两个子包，共 14 个数据类。serde 派生替代 Gson
//! 数据绑定；`#[serde(rename)]` 严格对照 `@SerializedName`。

pub mod dialog;
pub mod knowledge;
