//! 对应 Java `com.binarywang.wxjava.store.bean.ewaybill.PrintContentResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 PrintContentResponse；对应 Java com.binarywang.wxjava.store.bean.ewaybill.PrintContentResponse.java。
pub struct PrintContentResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 打印内容
    #[serde(rename = "print_content", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_content: Option<String>,
}
