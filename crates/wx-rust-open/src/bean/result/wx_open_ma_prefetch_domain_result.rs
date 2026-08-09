//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenMaPrefetchDomainResult.java`。
//!
//! 由 `scripts/gen_open_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMaPrefetchDomainResult {
    #[serde(rename = "errcode", default)]
    pub errcode: String,
    #[serde(rename = "errmsg", default)]
    pub errmsg: String,
    #[serde(rename = "prefetch_dns_domain", default)]
    pub prefetch_dns_domain: Vec<PreDnsDomain>,
    #[serde(rename = "size_limit", default)]
    pub size_limit: i32,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreDnsDomain {
    #[serde(rename = "url", default)]
    pub url: String,
    #[serde(rename = "status", default)]
    pub status: i32,
}
