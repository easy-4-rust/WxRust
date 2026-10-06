//! 对应 Java `com.binarywang.wxjava.store.bean.home.background.BackgroundGetResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::BackgroundApplyResult;

/// 微信小店 BackgroundGetResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.background.BackgroundGetResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BackgroundGetResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "img_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_url: Option<String>,
    #[serde(rename = "apply", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply: Option<BackgroundApplyResult>,
}
