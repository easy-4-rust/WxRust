//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftProductAddResponse.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 GiftProductAddResponse；对应 Java com.binarywang.wxjava.store.bean.product.GiftProductAddResponse.java。
pub struct GiftProductAddResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 赠品商品 ID
    #[serde(rename = "product_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
}
