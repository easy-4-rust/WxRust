//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthSubmitResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthSubmitResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "taskid", default)]
    pub task_id: String,
    #[serde(rename = "auth_url", default)]
    pub auth_url: String,
}
