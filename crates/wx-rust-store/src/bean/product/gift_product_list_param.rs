// 对应 Java `com.binarywang.wxjava.store.bean.product.GiftProductListParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GiftProductListParam；对应 Java com.binarywang.wxjava.store.bean.product.GiftProductListParam.java。
pub struct GiftProductListParam {
    /// 每页数量
    #[serde(rename = "page_size", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 status。
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}
