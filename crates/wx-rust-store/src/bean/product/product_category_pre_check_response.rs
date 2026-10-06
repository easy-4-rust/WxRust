/// 微信小店接口数据。对应 Java: com.binarywang.wxjava.store.bean.product.ProductCategoryPreCheckResponse
/// 微信小店 ProductCategoryPreCheckResponse 数据类型；对应 Java: com.binarywang.wxjava.store.bean.product.ProductCategoryPreCheckResponse。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProductCategoryPreCheckResponse {
    #[serde(rename = "errcode", skip_serializing_if = "Option::is_none")]
    pub err_code: Option<i32>,
    #[serde(rename = "errmsg", skip_serializing_if = "Option::is_none")]
    pub err_msg: Option<String>,
    #[serde(rename = "all_pass", skip_serializing_if = "Option::is_none")]
    pub all_pass: Option<bool>,
    #[serde(
        rename = "fail_reasons",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_reasons: Option<Vec<String>>,
}
