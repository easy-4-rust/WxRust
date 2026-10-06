//! 对应 Java `com.binarywang.wxjava.store.bean.fund.bank.BankCityInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BankCityInfo；对应 Java com.binarywang.wxjava.store.bean.fund.bank.BankCityInfo.java。
pub struct BankCityInfo {
    #[serde(rename = "city_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(rename = "city_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_code: Option<i32>,
    #[serde(rename = "bank_address_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_code: Option<String>,
}
