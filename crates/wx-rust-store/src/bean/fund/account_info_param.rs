//! 对应 Java `com.binarywang.wxjava.store.bean.fund.AccountInfoParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

use super::AccountInfo;

/// 微信小店 AccountInfoParam 数据类型；对应 Java com.binarywang.wxjava.store.bean.fund.AccountInfoParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfoParam {
    #[serde(rename = "account_info", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_info: Option<AccountInfo>,
}
