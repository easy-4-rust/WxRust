//! 对应 Java `com.binarywang.wxjava.store.bean.order.DecodeSensitiveInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::{DecodeAddressInfo, VirtualNumberInfo};

/// 微信小店 DecodeSensitiveInfoResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.DecodeSensitiveInfoResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecodeSensitiveInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<DecodeAddressInfo>,
    #[serde(rename = "virtual_number_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_number_info: Option<VirtualNumberInfo>,
}
