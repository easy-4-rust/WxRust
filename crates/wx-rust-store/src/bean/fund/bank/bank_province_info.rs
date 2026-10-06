//! 对应 Java `com.binarywang.wxjava.store.bean.fund.bank.BankProvinceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 BankProvinceInfo；对应 Java com.binarywang.wxjava.store.bean.fund.bank.BankProvinceInfo.java。
pub struct BankProvinceInfo {
    #[serde(rename = "province_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_name: Option<String>,
    #[serde(rename = "province_code", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province_code: Option<i32>,
}
