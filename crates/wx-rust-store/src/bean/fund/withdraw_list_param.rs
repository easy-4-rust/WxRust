//! 对应 Java `com.binarywang.wxjava.store.bean.fund.WithdrawListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WithdrawListParam；对应 Java com.binarywang.wxjava.store.bean.fund.WithdrawListParam.java。
pub struct WithdrawListParam {
    #[serde(rename = "page_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "start_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    #[serde(rename = "end_time", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}
