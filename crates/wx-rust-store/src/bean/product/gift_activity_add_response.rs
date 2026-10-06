//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftActivityAddResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GiftActivityAddResponse；对应 Java com.binarywang.wxjava.store.bean.product.GiftActivityAddResponse.java。
pub struct GiftActivityAddResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 活动 ID
    #[serde(rename = "activity_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_id: Option<String>,
}
