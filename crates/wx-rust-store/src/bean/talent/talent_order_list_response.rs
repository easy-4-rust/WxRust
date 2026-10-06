//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentOrderListResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentOrderListResponse；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderListResponse.java。
pub struct TalentOrderListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 佣金单列表
    #[serde(rename = "order_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_list: Option<Vec<TalentOrderInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 has_more。
    #[serde(rename = "has_more", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 微信小店 TalentOrderInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderInfo {
    /// 商品 SKU 标识。
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    /// 本地生活特殊标识。
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_id: Option<String>,
    /// 佣金单号
    #[serde(rename = "order_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentOrderListResponse#OrderInfo
/// 微信小店 TalentOrderListResponseOrderInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentOrderListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentOrderListResponseOrderInfo {
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// 上游字段 sku_id。
    #[serde(rename = "sku_id", skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<String>,
    /// 上游字段 special_id。
    #[serde(
        rename = "special_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub special_id: Option<String>,
}
