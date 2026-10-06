//! 对应 Java `com.binarywang.wxjava.store.bean.home.window.WindowProductSettingResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::WindowProductSetting;

/// 微信小店 WindowProductSettingResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.home.window.WindowProductSettingResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WindowProductSettingResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "products", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<WindowProductSetting>>,
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
}
