//! 小程序信息（旧版）。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.auth.WxOpenMiniProgramInfo`。
//! 无 Gson adapter，反射线格式为 Java 字段名（`network`/`categories`/
//! `visitStatus`）。
//!
//! ADAPTED：Java `categories` 为 `List<Pair<String, String>>`（commons-lang
//! 三元组库），Gson 反射输出 `ImmutablePair` 的 `left`/`right` 字段；Rust 以
//! 同线格式的 `StringPair` 表达。该字段无线格式 golden，Wave 2 结合授权方
//! 信息接口实测校准。

use std::collections::HashMap;

/// 小程序信息（旧版）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenMiniProgramInfo {
    /// 域名信息（对应 Java `network`：域名 → 域名列表）。
    #[serde(rename = "network", default)]
    pub network: Option<HashMap<String, Vec<String>>>,
    /// 类目键值对列表（对应 Java `categories`，镜像 `Pair` 的 left/right 线格式）。
    #[serde(rename = "categories", default)]
    pub categories: Option<Vec<StringPair>>,
    /// 访问状态。
    #[serde(rename = "visitStatus", default)]
    pub visit_status: Option<i32>,
}

/// 字符串键值对（镜像 Java `org.apache.commons.lang3.tuple.Pair` 的
/// Gson 反射线格式 `{"left": ..., "right": ...}`）。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StringPair {
    #[serde(rename = "left", default)]
    pub left: Option<String>,
    #[serde(rename = "right", default)]
    pub right: Option<String>,
}
