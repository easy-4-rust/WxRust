//! 对应 Java `com.binarywang.wxjava.store.bean.limit.LimitTaskListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::LimitTaskInfo;

/// 微信小店 LimitTaskListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.limit.LimitTaskListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitTaskListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "limited_discount_tasks", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<LimitTaskInfo>>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
}
