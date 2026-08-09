//! 对应 Java `me.chanjar.weixin.channel.bean.fund.AccountInfoParam.java`。
//!
//! 由 `scripts/gen_channel_bean_structs.py` 从 Java 数据类生成（@JsonProperty 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInfoParam {
    #[serde(rename = "account_info", default)]
    pub account_info: AccountInfo,
}
