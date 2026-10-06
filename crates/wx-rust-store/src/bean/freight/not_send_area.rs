//! 对应 Java `com.binarywang.wxjava.store.bean.freight.NotSendArea.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use crate::bean::base::AddressInfo;

/// 微信小店 NotSendArea 数据类型；对应 Java com.binarywang.wxjava.store.bean.freight.NotSendArea.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NotSendArea {
    #[serde(rename = "address_infos", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_infos: Option<Vec<AddressInfo>>,
}
