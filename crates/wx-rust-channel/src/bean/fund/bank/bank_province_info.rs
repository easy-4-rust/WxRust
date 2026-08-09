//! 对应 Java `me.chanjar.weixin.channel.bean.fund.bank.BankProvinceInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::fund::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankProvinceInfo {
    #[serde(rename = "province_name", default)]
    pub province_name: String,
    #[serde(rename = "province_code", default)]
    pub province_code: i32,
}
