//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftProductGetResponse.java`。

#[allow(unused_imports)]
use super::gift_product_info::GiftProductInfo;

/// 微信小店 GiftProductGetResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftProductGetResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductGetResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 赠品信息
    #[serde(rename = "gift_product_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_product_info: Option<GiftProductInfo>,
    /// 上游字段 product。
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<GiftProductInfo>,
    /// 上游字段 edit_product。
    #[serde(
        rename = "edit_product",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edit_product: Option<GiftProductInfo>,
}
