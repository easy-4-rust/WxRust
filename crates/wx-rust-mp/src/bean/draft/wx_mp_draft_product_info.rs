//! 对应 Java `bean.draft.WxMpDraftProductInfo`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxMpDraftProductInfo {
    #[serde(rename = "footer_product_info", default)]
    pub footer_product_info: FooterProductInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FooterProductInfo {
    #[serde(rename = "product_key", default)]
    pub product_key: String,
}
