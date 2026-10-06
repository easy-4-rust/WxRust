//! 对应 Java `com.binarywang.wxjava.store.bean.home.window.WindowProductIndexParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WindowProductIndexParam；对应 Java com.binarywang.wxjava.store.bean.home.window.WindowProductIndexParam.java。
pub struct WindowProductIndexParam {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "index_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_num: Option<i32>,
}
