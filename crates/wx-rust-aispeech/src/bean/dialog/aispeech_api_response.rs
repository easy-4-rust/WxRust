//! 对应 Java `me.chanjar.weixin.aispeech.bean.dialog.AispeechApiResponse.java`。

use serde::{Deserialize, Serialize};

/// 对话 API 统一响应包装。
///
/// 对应 Java 泛型类 `AispeechApiResponse<T>`：`code` 为业务错误码
/// （0 表示成功），`msg` 为错误信息，`request_id` 为请求标识，`data` 为
/// 业务数据（`@SerializedName("request_id")`）。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AispeechApiResponse<T> {
    /// 业务错误码（0 成功；`code == null` 视为失败，见 `ensureSuccess`）
    pub code: Option<i32>,
    /// 错误信息
    pub msg: Option<String>,
    /// 请求标识（对应 Java `@SerializedName("request_id")`）
    #[serde(rename = "request_id", default)]
    pub request_id: Option<String>,
    /// 业务数据
    pub data: Option<T>,
}

impl<T> AispeechApiResponse<T> {
    /// 从 JSON 解析响应（对应 Java `WxGsonBuilder.fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_str(json).map_err(|e| format!("AispeechApiResponse 解析失败: {e}"))
    }
}
