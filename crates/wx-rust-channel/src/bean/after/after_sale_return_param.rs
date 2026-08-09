//! 对应 Java `me.chanjar.weixin.channel.bean.after.AfterSaleReturnParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleReturnParam {
    #[serde(rename = "aftersale_id", default)]
    pub after_sale_id: i64,
    #[serde(rename = "out_aftersale_id", default)]
    pub out_after_sale_id: String,
    #[serde(rename = "address_info", default)]
    pub address_info: AddressInfo,
}
