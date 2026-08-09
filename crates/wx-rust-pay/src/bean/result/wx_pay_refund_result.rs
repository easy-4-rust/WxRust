//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayRefundResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxPayRefundResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_code"
    )]
    pub return_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "return_msg"
    )]
    pub return_msg: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "result_code"
    )]
    pub result_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "err_code")]
    pub err_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "err_code_des"
    )]
    pub err_code_des: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "error_code"
    )]
    pub error_code: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "error_message"
    )]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "appid")]
    pub appid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mch_id")]
    pub mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_appid")]
    pub sub_app_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_mch_id"
    )]
    pub sub_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "nonce_str")]
    pub nonce_str: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sign")]
    pub sign: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "xmlString")]
    pub xml_string: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "transaction_id"
    )]
    pub transaction_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_trade_no"
    )]
    pub out_trade_no: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "out_refund_no"
    )]
    pub out_refund_no: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "refund_id")]
    pub refund_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_fee"
    )]
    pub refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_refund_fee"
    )]
    pub settlement_refund_fee: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "settlement_total_fee"
    )]
    pub settlement_total_fee: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fee_type")]
    pub fee_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cash_fee")]
    pub cash_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cash_fee_type"
    )]
    pub cash_fee_type: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cash_refund_fee"
    )]
    pub cash_refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_refund_count"
    )]
    pub coupon_refund_count: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_refund_fee"
    )]
    pub coupon_refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "promotion_detail"
    )]
    pub promotion_detail_string: Option<String>,
    #[serde(default, rename = "promotionDetails")]
    pub promotion_details: Vec<WxPayRefundPromotionDetail>,
    #[serde(default, rename = "refundCoupons")]
    pub refund_coupons: Vec<WxPayRefundCouponInfo>,
}

/// 退款结果的组合逻辑（对应 Java `WxPayRefundResult`）。
impl WxPayRefundResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self =
            quick_xml::de::from_str(xml).map_err(|e| format!("WxPayRefundResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_promotion_details();
        v.compose_refund_coupons();
        Ok(v)
    }

    /// 解析 `promotion_detail`（内嵌 JSON 串，对应 Java `composePromotionDetails`）。
    pub fn compose_promotion_details(&mut self) {
        let Some(s) = self.promotion_detail_string.as_deref() else {
            return;
        };
        if s.is_empty() {
            return;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            if let Some(arr) = v.get("promotion_detail") {
                if let Ok(list) =
                    serde_json::from_value::<Vec<WxPayRefundPromotionDetail>>(arr.clone())
                {
                    self.promotion_details = list;
                }
            }
        }
    }

    /// 组装 `refund_coupons`（对应 Java `composeRefundCoupons`：
    /// `xml/coupon_refund_id_{i}`/`coupon_refund_fee_{i}`/`coupon_type_{i}`）。
    pub fn compose_refund_coupons(&mut self) {
        let count = self.coupon_refund_count.unwrap_or(0);
        if count <= 0 {
            return;
        }
        let Some(xml) = self.xml_string.as_deref() else {
            return;
        };
        let map = match crate::bean::xml::root_children_map(xml) {
            Ok(m) => m,
            Err(_) => return,
        };
        self.refund_coupons = (0..count)
            .map(|i| WxPayRefundCouponInfo {
                coupon_refund_id: map.get(&format!("coupon_refund_id_{i}")).cloned(),
                coupon_refund_fee: map
                    .get(&format!("coupon_refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
            })
            .collect();
    }
}
