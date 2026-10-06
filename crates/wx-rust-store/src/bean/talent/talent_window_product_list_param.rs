//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentWindowProductListParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentWindowProductListParam；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductListParam.java。
pub struct TalentWindowProductListParam {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 page_index。
    #[serde(
        rename = "page_index",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub page_index: Option<i32>,
    /// 上游字段 last_buffer。
    #[serde(
        rename = "last_buffer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_buffer: Option<String>,
}
