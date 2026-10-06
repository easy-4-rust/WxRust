//! 对应 Java `com.binarywang.wxjava.store.bean.after.AfterSaleReturnParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 AfterSaleReturnParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.after.AfterSaleReturnParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AfterSaleReturnParam {
    #[serde(rename = "aftersale_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_sale_id: Option<i64>,
    #[serde(rename = "out_aftersale_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_after_sale_id: Option<String>,
    #[serde(rename = "address_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info: Option<AddressInfo>,
}
