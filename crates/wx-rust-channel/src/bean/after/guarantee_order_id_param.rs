//! 对应 Java `me.chanjar.weixin.channel.bean.after.GuaranteeOrderIdParam.java`。

#[allow(unused_imports)]
use super::*;

/// 保障单号参数。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GuaranteeOrderIdParam {
    /// 保障单号。
    #[serde(rename = "guarantee_order_id", default)]
    pub guarantee_order_id: String,
}
