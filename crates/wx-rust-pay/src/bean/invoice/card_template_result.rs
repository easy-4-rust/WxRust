//! 电子发票卡券模板结果。
//!
//! 对应 Java `com.github.binarywang.wxpay.bean.invoice.CardTemplateResult`。

use serde::{Deserialize, Serialize};

/// 电子发票卡券模板结果。
///
/// 对应 Java: `CardTemplateResult`
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CardTemplateResult {
    /// 卡券 AppID。
    #[serde(rename = "card_appid", skip_serializing_if = "Option::is_none")]
    pub card_appid: Option<String>,

    /// 卡券 ID。
    #[serde(rename = "card_id", skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
}
