//! 对应 Java `com.binarywang.wxjava.store.bean.order.QualityInsepctInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 QualityInsepctInfo；对应 Java com.binarywang.wxjava.store.bean.order.QualityInsepctInfo.java。
pub struct QualityInsepctInfo {
    #[serde(rename = "inspect_status", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inspect_status: Option<i32>,
}
