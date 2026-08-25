//! 对应 Java `me.chanjar.weixin.channel.bean.product.GiftProductListParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductListParam {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
}
