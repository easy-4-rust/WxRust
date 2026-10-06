//! 对应 Java `com.binarywang.wxjava.store.bean.talent.TalentWindowProductListResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 TalentWindowProductListResponse；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductListResponse.java。
pub struct TalentWindowProductListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 橱窗商品列表
    #[serde(rename = "product_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_list: Option<Vec<TalentWindowProductInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 products。
    #[serde(rename = "products", skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<TalentWindowProductListResponseProductInfo>>,
    /// 上游字段 last_buffer。
    #[serde(
        rename = "last_buffer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_buffer: Option<String>,
}

/// 微信小店 TalentWindowProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductInfo {
    /// 商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// 商品名称
    #[serde(rename = "product_name", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: talent.TalentWindowProductListResponse#ProductInfo
/// 微信小店 TalentWindowProductListResponseProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.talent.TalentWindowProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TalentWindowProductListResponseProductInfo {
    /// 上游字段 product_id。
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
    /// 上游字段 appid。
    #[serde(rename = "appid", skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    /// 上游字段 product_source。
    #[serde(
        rename = "product_source",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_source: Option<i32>,
    /// 上游字段 out_product_id。
    #[serde(
        rename = "out_product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub out_product_id: Option<String>,
}
