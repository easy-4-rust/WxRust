//! 对应 Java `me.chanjar.weixin.channel.bean.order.DecodeSensitiveInfoResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::WxChannelBaseResponse;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecodeSensitiveInfoResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "address_info", default)]
    pub address_info: DecodeAddressInfo,
    #[serde(rename = "virtual_number_info", default)]
    pub virtual_number_info: VirtualNumberInfo,
}
