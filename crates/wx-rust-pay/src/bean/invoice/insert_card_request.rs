//! 将电子发票插入微信用户卡包请求。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.InsertCardRequest`。

use serde::{Deserialize, Serialize};

/// 将电子发票插入微信用户卡包请求。
///
/// 对应 Java: `InsertCardRequest`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InsertCardRequest {
    /// 子商户号。
    #[serde(rename = "sub_mchid", skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,

    /// 场景。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,

    /// 开票申请单号。
    #[serde(rename = "fapiao_apply_id", skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,

    /// 购买方信息。
    #[serde(rename = "buyer_information", skip_serializing_if = "Option::is_none")]
    pub buyer_information: Option<super::buyer_information::BuyerInformation>,

    /// 发票卡券信息。
    #[serde(
        rename = "fapiao_card_information",
        skip_serializing_if = "Option::is_none"
    )]
    pub fapiao_card_information: Option<Vec<serde_json::Value>>,
}
