//! 对应 Java `bean.comment.WxMpCommentListVo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpCommentListVo {
    #[serde(rename = "total", default)]
    pub total: i32,
    #[serde(rename = "comment", default)]
    pub comment: Vec<WxMpComment>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Reply {
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "content", default)]
    pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpComment {
    #[serde(rename = "user_comment_id", default)]
    pub user_comment_id: i32,
    #[serde(rename = "openid", default)]
    pub openid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: String,
    #[serde(rename = "content", default)]
    pub content: String,
    #[serde(rename = "comment_type", default)]
    pub comment_type: i32,
    #[serde(rename = "reply", default)]
    pub reply: Reply,
}

impl WxMpCommentListVo {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxMpCommentListVo 解析失败: {e}"))
    }
}
