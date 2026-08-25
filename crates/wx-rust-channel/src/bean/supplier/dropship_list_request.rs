//! 对应 Java `me.chanjar.weixin.channel.bean.supplier.DropshipListRequest.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DropshipListRequest {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    pub page_size: i32,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    pub next_key: String,
    /// 供货商 ID
    #[serde(rename = "supplier_id", default)]
    pub supplier_id: String,
}
