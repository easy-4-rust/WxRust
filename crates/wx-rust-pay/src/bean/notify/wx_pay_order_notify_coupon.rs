//! 对应 Java `com.github.binarywang.wxpay.bean.notify.WxPayOrderNotifyCoupon.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxPayOrderNotifyCoupon {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "couponId")]
    pub coupon_id: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "couponType"
    )]
    pub coupon_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "couponFee")]
    pub coupon_fee: Option<i32>,
}

/// 代金券组合辅助（对应 Java `WxPayOrderNotifyCoupon.toMap(int index)`）。
impl WxPayOrderNotifyCoupon {
    /// 以 `coupon_id_{index}`/`coupon_type_{index}`/`coupon_fee_{index}` 键导出。
    pub fn to_map(&self, index: usize) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(v) = self.coupon_id.as_deref() {
            map.insert(format!("coupon_id_{index}"), v.to_string());
        }
        if let Some(v) = self.coupon_type.as_deref() {
            map.insert(format!("coupon_type_{index}"), v.to_string());
        }
        map.insert(
            format!("coupon_fee_{index}"),
            format!("{}", self.coupon_fee.unwrap_or_default()),
        );
        map
    }
}
