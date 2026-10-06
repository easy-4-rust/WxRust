//! 对应 Java `com.binarywang.wxjava.store.bean.address.AddressCodeResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::AddressCode;

/// 微信小店 AddressCodeResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.address.AddressCodeResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressCodeResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "addrs_msg", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<AddressCode>,
    #[serde(rename = "next_level_addrs", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<AddressCode>>,
}
