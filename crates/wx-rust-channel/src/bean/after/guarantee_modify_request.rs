//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeModifyRequest.java`。

#[allow(unused_imports)]
use super::*;

/// 商家协商保障单请求参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeModifyRequest {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
    /// 商品破损程度。
    #[serde(rename = "bad_level", default)]
    pub bad_level: i32,
    /// 商家协商备注。
    #[serde(rename = "merchant_remark", default)]
    pub merchant_remark: String,
}
