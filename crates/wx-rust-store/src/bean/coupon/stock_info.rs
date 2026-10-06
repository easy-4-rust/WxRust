//! 对应 Java `com.binarywang.wxjava.store.bean.coupon.StockInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 StockInfo；对应 Java com.binarywang.wxjava.store.bean.coupon.StockInfo.java。
pub struct StockInfo {
    #[serde(rename = "issued_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_num: Option<i32>,
    #[serde(rename = "receive_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receive_num: Option<i32>,
    #[serde(rename = "used_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_num: Option<i32>,
}
