//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.DialogQueryRequest.java`。

use serde::{Deserialize, Serialize};

/// 对话查询请求。
///
/// 对应 Java `DialogQueryRequest`：`query(DialogQueryRequest)` 的入参，
/// 发送前经 AES-CBC 加密为密文报文。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DialogQueryRequest {
    /// 用户提问
    #[serde(default)]
    pub query: Option<String>,
    /// 环境（如 `online`）
    #[serde(default)]
    pub env: Option<String>,
    /// 优先技能列表（对应 Java `@SerializedName("first_priority_skills")`）
    #[serde(rename = "first_priority_skills", default)]
    pub first_priority_skills: Option<Vec<String>>,
    /// 次级优先技能列表（对应 Java `@SerializedName("second_priority_skills")`）
    #[serde(rename = "second_priority_skills", default)]
    pub second_priority_skills: Option<Vec<String>>,
    /// 用户名（对应 Java `@SerializedName("user_name")`）
    #[serde(rename = "user_name", default)]
    pub user_name: Option<String>,
    /// 用户头像
    #[serde(default)]
    pub avatar: Option<String>,
    /// 用户 id
    #[serde(default)]
    pub userid: Option<String>,
}
