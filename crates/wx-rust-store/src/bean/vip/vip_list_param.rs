//! 对应 Java `com.binarywang.wxjava.store.bean.vip.VipListParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 VipListParam；对应 Java com.binarywang.wxjava.store.bean.vip.VipListParam.java。
pub struct VipListParam {
    #[serde(rename = "need_phone_number", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    #[serde(rename = "page_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_num: Option<i32>,
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
}
