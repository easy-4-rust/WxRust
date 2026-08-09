//! 对应 Java `me.chanjar.weixin.cp.bean.oa.WxCpApprovalInfo.java`。
//!
//! 由 `scripts/gen_cp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxCpApprovalInfo {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "sp_no_list", default)]
    pub sp_no_list: Vec<String>,
    #[serde(rename = "next_cursor", default)]
    pub next_cursor: i32,
    #[serde(rename = "new_next_cursor", default)]
    pub new_next_cursor: String,
}
