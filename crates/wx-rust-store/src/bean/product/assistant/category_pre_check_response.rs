//! 对应 Java `com.binarywang.wxjava.store.bean.product.assistant.CategoryPreCheckResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 CategoryPreCheckResponse；对应 Java com.binarywang.wxjava.store.bean.product.assistant.CategoryPreCheckResponse.java。
pub struct CategoryPreCheckResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 是否可用
    #[serde(rename = "available", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// 上游字段 all_pass。
    #[serde(rename = "all_pass", skip_serializing_if = "Option::is_none")]
    pub all_pass: Option<bool>,
    /// 上游字段 fail_reasons。
    #[serde(
        rename = "fail_reasons",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fail_reasons: Option<Vec<String>>,
}
