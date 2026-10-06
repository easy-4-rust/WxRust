//! 对应 Java `com.binarywang.wxjava.store.bean.limit.LimitTaskUpdateResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 LimitTaskUpdateResponse；对应 Java com.binarywang.wxjava.store.bean.limit.LimitTaskUpdateResponse.java。
pub struct LimitTaskUpdateResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 上游字段 task_id。
    #[serde(rename = "task_id", skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// 上游字段 title。
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
