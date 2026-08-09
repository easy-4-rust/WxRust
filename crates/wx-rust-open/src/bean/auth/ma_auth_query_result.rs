//! 对应 Java `me.chanjar.weixin.open.bean.auth.MaAuthQueryResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaAuthQueryResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "appid", default)]
    pub app_id: String,
    #[serde(rename = "task_status", default)]
    pub task_status: i32,
    #[serde(rename = "auth_url", default)]
    pub auth_url: String,
    #[serde(rename = "apply_status", default)]
    pub apply_status: i32,
    #[serde(rename = "orderid", default)]
    pub order_id: String,
    #[serde(rename = "refill_reason", default)]
    pub refill_reason: String,
    #[serde(rename = "fail_reason", default)]
    pub fail_reason: String,
    #[serde(rename = "dispatch_info", default)]
    pub dispatch_info: MaAuthQueryResultDispatchInfo,
}
