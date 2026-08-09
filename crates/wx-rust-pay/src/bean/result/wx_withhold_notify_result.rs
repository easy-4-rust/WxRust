//! 对应 Java `com.github.binarywang.wxpay.bean.result.WxWithholdNotifyResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;
#[allow(unused_imports)]
use crate::bean::notify::WxPayOrderNotifyCoupon;
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename = "xml")]
pub struct WxWithholdNotifyResult {
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "openid")]
    pub open_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_openid"
    )]
    pub sub_open_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "is_subscribe"
    )]
    pub is_subscribe: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "sub_is_subscribe"
    )]
    pub sub_is_subscribe: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "bank_type")]
    pub bank_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "total_fee")]
    pub total_fee: Option<i32>,
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
        rename = "trade_state"
    )]
    pub trade_state: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_fee"
    )]
    pub coupon_fee: Option<i32>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "coupon_count"
    )]
    pub coupon_count: Option<i32>,
    #[serde(default, rename = "couponList")]
    pub coupon_list: Vec<WxPayOrderNotifyCoupon>,
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attach")]
    pub attach: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "time_end")]
    pub time_end: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "contract_id"
    )]
    pub contract_id: Option<String>,
}

/// 委托代扣支付结果通知（XML，对应 Java `WxWithholdNotifyResult`）。
impl WxWithholdNotifyResult {
    /// 从 XML 解析（对应 Java `fromXML`：结构字段 + 原始报文 + 代金券列表组装）。
    pub fn from_xml(xml: &str) -> Result<Self, String> {
        let mut v: Self = quick_xml::de::from_str(xml)
            .map_err(|e| format!("WxWithholdNotifyResult 解析失败: {e}"))?;
        v.xml_string = Some(xml.to_string());
        v.compose_coupons();
        Ok(v)
    }

    /// 组装 `coupon_list`（对应 Java `composeCoupons`）。
    pub fn compose_coupons(&mut self) {
        let count = self.coupon_count.unwrap_or(0);
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
        self.coupon_list = (0..count)
            .map(|i| WxPayOrderNotifyCoupon {
                coupon_id: map.get(&format!("coupon_id_{i}")).cloned(),
                coupon_type: map.get(&format!("coupon_type_{i}")).cloned(),
                coupon_fee: map
                    .get(&format!("coupon_fee_{i}"))
                    .and_then(|s| s.trim().parse().ok()),
            })
            .collect();
    }
}
