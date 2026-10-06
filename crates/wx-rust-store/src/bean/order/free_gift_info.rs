//! 对应 Java `com.binarywang.wxjava.store.bean.order.FreeGiftInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::MainProductInfo;

/// 微信小店 FreeGiftInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.order.FreeGiftInfo.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FreeGiftInfo {
    #[serde(rename = "main_product_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_product_list: Option<Vec<MainProductInfo>>,
}
