//! 对应 Java `com.binarywang.wxjava.store.bean.window.response.GetWindowProductListResponse.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GetWindowProductListResponse；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductListResponse.java。
pub struct GetWindowProductListResponse {
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    #[serde(rename = "products", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<ProductInfo>>,
    #[serde(rename = "last_buffer", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_buffer: Option<String>,
    #[serde(rename = "total_num", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
}

/// 微信小店 ProductInfo 数据类型；对应 Java com.binarywang.wxjava.store.bean.window.response.GetWindowProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductInfo {
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(rename = "appid", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
}
