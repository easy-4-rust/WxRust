//! 对应 Java `com.binarywang.wxjava.store.bean.product.GiftProductListResponse.java`。

#[allow(unused_imports)]
use super::gift_product_info::GiftProductInfo;

/// 微信小店 GiftProductListResponse 数据类型；对应 Java com.binarywang.wxjava.store.bean.product.GiftProductListResponse.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GiftProductListResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 赠品列表
    #[serde(rename = "gift_product_list", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift_product_list: Option<Vec<GiftProductInfo>>,
    /// 翻页上下文
    #[serde(rename = "next_key", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_key: Option<String>,
    /// 上游字段 total_num。
    #[serde(rename = "total_num", skip_serializing_if = "Option::is_none")]
    pub total_num: Option<i32>,
    /// 上游字段 product_ids。
    #[serde(
        rename = "product_ids",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ids: Option<Vec<String>>,
}
