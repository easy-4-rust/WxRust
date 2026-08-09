//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalComment.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalComment {
    #[serde(rename = "commentUserInfo", default)]
    pub comment_user_info: crate::bean::oa::wx_cp_operator::WxCpOperator,
    #[serde(rename = "commenttime", default)]
    pub comment_time: i64,
    #[serde(rename = "commentid", default)]
    pub comment_id: String,
    #[serde(rename = "commentcontent", default)]
    pub comment_content: String,
    #[serde(rename = "media_id", default)]
    pub media_ids: Vec<String>,
}
