//! 对应 Java `com.binarywang.wxjava.store.bean.cooperation.CooperationSharerParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CooperationSharerParam；对应 Java com.binarywang.wxjava.store.bean.cooperation.CooperationSharerParam.java。
pub struct CooperationSharerParam {
    #[serde(rename = "sharer_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_id: Option<String>,
    #[serde(rename = "sharer_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharer_type: Option<i32>,
}
