//! 对应 Java `com.binarywang.wxjava.store.bean.base.OffsetParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 OffsetParam；对应 Java com.binarywang.wxjava.store.bean.base.OffsetParam.java。
pub struct OffsetParam {
    #[serde(rename = "offset", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(rename = "limit", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}
