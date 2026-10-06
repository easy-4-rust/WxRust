//! 对应 Java `com.binarywang.wxjava.store.bean.brand.BrandParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::Brand;

/// 微信小店 BrandParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.brand.BrandParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandParam {
    #[serde(rename = "brand", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<Brand>,
}
