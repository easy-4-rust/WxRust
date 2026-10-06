//! 对应 Java `com.binarywang.wxjava.store.bean.product.ExtraServiceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 ExtraServiceInfo；对应 Java com.binarywang.wxjava.store.bean.product.ExtraServiceInfo.java。
pub struct ExtraServiceInfo {
    #[serde(rename = "seven_day_return", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seven_day_return: Option<i32>,
    #[serde(rename = "pay_after_use", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_after_use: Option<i32>,
    #[serde(rename = "freight_insurance", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_insurance: Option<i32>,
    #[serde(rename = "fake_one_pay_three", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fake_one_pay_three: Option<i32>,
    #[serde(rename = "damage_guarantee", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage_guarantee: Option<i32>,
}
