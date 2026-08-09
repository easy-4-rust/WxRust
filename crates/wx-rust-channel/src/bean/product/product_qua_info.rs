//! 对应 Java `me.chanjar.weixin.channel.bean.product.ProductQuaInfo.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductQuaInfo {
    #[serde(rename = "qua_id", default)]
    pub qua_id: String,
    #[serde(rename = "qua_url", default)]
    pub qua_url: Vec<String>,
}
