//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxPayRefundQueryResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxPayRefundQueryResult {
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
        rename = "device_info"
    )]
    pub device_info: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_fee"
    )]
    pub refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_refund_fee"
    )]
    pub coupon_refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "cash_refund_fee"
    )]
    pub cash_refund_fee: Option<i32>,
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
        rename = "refund_count"
    )]
    pub refund_count: Option<i32>,
    #[serde(default, rename = "refundRecords")]
    pub refund_records: Vec<RefundRecord>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "promotion_detail"
    )]
    pub promotion_detail_string: Option<String>,
    #[serde(default, rename = "promotionDetails")]
    pub promotion_details: Vec<WxPayRefundPromotionDetail>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefundRecord {
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
        rename = "refund_channel"
    )]
    pub refund_channel: Option<String>,
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
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_account"
    )]
    pub refund_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_refund_fee"
    )]
    pub coupon_refund_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_refund_count"
    )]
    pub coupon_refund_count: Option<i32>,
    #[serde(default, rename = "refundCoupons")]
    pub refund_coupons: Vec<WxPayRefundCouponInfo>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_status"
    )]
    pub refund_status: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_recv_accout"
    )]
    pub refund_recv_account: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "refund_success_time"
    )]
    pub refund_success_time: Option<String>,
}

/// 查询退款结果的组合逻辑（对应 Java `WxPayRefundQueryResult`）。
impl WxPayRefundQueryResult {
    /// 从 XML 解析（对应 Java `fromXML`）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxPayRefundQueryResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_refund_records();
        v.compose_promotion_details();
        Ok(v)
    }

    /// 组装 `refund_records`（对应 Java `composeRefundRecords`：
    /// `xml/refund_*_{i}` 与嵌套 `xml/coupon_refund_*_{i}_{j}` 索引字段）。
    pub fn compose_refund_records(&mut self) {
        let count = self.refund_count.unwrap_or(0);
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
        let mut records = Vec::with_capacity(count as usize);
        for i in 0..count {
            let coupon_refund_count = map
                .get(&format!("coupon_refund_count_{i}"))
                .and_then(|s| s.trim().parse::<i32>().ok());
            let mut coupons = Vec::new();
            if let Some(c) = coupon_refund_count {
                if c > 0 {
                    for j in 0..c {
                        coupons.push(WxPayRefundCouponInfo {
                            coupon_refund_id: map
                                .get(&format!("coupon_refund_id_{i}_{j}"))
                                .cloned(),
                            coupon_refund_fee: map
                                .get(&format!("coupon_refund_fee_{i}_{j}"))
                                .and_then(|s| s.trim().parse().ok()),
                            coupon_type: map.get(&format!("coupon_type_{i}_{j}")).cloned(),
                        });
                    }
                }
            }
            records.push(RefundRecord {
                out_refund_no: map.get(&format!("out_refund_no_{i}")).cloned(),
                refund_id: map.get(&format!("refund_id_{i}")).cloned(),
                refund_channel: map.get(&format!("refund_channel_{i}")).cloned(),
                refund_fee: map
                    .get(&format!("refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                settlement_refund_fee: map
                    .get(&format!("settlement_refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                coupon_refund_fee: map
                    .get(&format!("coupon_refund_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
                coupon_refund_count,
                refund_account: map.get(&format!("refund_account_{i}")).cloned(),
                refund_status: map.get(&format!("refund_status_{i}")).cloned(),
                refund_recv_account: map.get(&format!("refund_recv_accout_{i}")).cloned(),
                refund_success_time: map.get(&format!("refund_success_time_{i}")).cloned(),
                refund_coupons: coupons,
            });
        }
        self.refund_records = records;
    }

    /// 解析 `promotion_detail`（XML 元素内嵌 JSON 串，对应 Java `composePromotionDetails`：
    /// 取 JSON 的 `promotion_detail` 数组）。
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
}
