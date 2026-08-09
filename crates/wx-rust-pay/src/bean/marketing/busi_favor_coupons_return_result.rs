//! 对应 Java `com.github.binarywang.wxpay.bean.marketing.BusiFavorCouponsReturnResult.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusiFavorCouponsReturnResult {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "wechatpay_return_time"
    )]
    pub wechatpay_return_time: Option<String>,
}
