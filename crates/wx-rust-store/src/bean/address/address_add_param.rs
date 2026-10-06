//! 对应 Java `com.binarywang.wxjava.store.bean.address.AddressAddParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::AddressDetail;

/// 微信小店 AddressAddParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.address.AddressAddParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AddressAddParam {
    #[serde(rename = "address_detail", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_detail: Option<AddressDetail>,
}
