//! 对应 Java `com.binarywang.wxjava.store.bean.product.LimitInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 LimitInfo；对应 Java com.binarywang.wxjava.store.bean.product.LimitInfo.java。
pub struct LimitInfo {
    #[serde(rename = "period_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_type: Option<i32>,
    #[serde(rename = "limited_buy_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>,
}
