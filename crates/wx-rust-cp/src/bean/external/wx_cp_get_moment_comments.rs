//! 对应 Java `me.chanjar.weixin.cp.bean.external.WxCpGetMomentComments.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpGetMomentComments {
    #[serde(rename = "errcode", default)]
    pub errcode: i64,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "comment_list", default)]
    pub comment_list: Vec<CommentLikeItem>,
    #[serde(rename = "like_list", default)]
    pub like_list: Vec<CommentLikeItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CommentLikeItem {
    #[serde(rename = "external_userid", default)]
    pub external_user_id: String,
    #[serde(rename = "userid", default)]
    pub userid: String,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
}

impl WxCpGetMomentComments {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("WxCpGetMomentComments 解析失败: {e}"))
    }
}

impl WxCpGetMomentComments {
    /// 序列化为 JSON（对应 Java `toJson`）。
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("WxCpGetMomentComments 序列化失败: {e}"))
    }
}
