//! 对应 Java `com.github.binarywang.wxpay.bean.profitsharing.result.ProfitSharingMerchantRatioQueryV3Result.java`。
//!
//! 由 `scripts/gen_pay_bean_structs.py` 从 Java 数据类生成（@SerializedName/@XStreamAlias 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProfitSharingMerchantRatioQueryV3Result {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sub_mchid")]
    pub sub_mch_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "max_ratio")]
    pub max_ratio: Option<i32>,
}
