//! 卡券 API 签名。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.WxCardApiSignature`。

use serde::{Deserialize, Serialize};

/// 卡券 API 签名结果。
///
/// 用于微信卡券的 JS-SDK 签名（`wx.addCard` 等）。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WxCardApiSignature {
    /// 公众号 appId
    pub app_id: String,

    /// 卡券 ID
    pub card_id: String,

    /// 卡券类型
    pub card_type: String,

    /// 门店 ID（可选）
    pub location_id: Option<String>,

    /// 卡券 Code（可选）
    pub code: Option<String>,

    /// 用户 openId（可选）
    pub open_id: Option<String>,

    /// 时间戳（秒）
    pub timestamp: Option<i64>,

    /// 随机串
    pub nonce_str: String,

    /// 签名值
    pub signature: String,
}

impl WxCardApiSignature {
    /// 构建卡券签名。
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        app_id: impl Into<String>,
        card_id: impl Into<String>,
        card_type: impl Into<String>,
        location_id: Option<String>,
        code: Option<String>,
        open_id: Option<String>,
        timestamp: Option<i64>,
        nonce_str: impl Into<String>,
        signature: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            card_id: card_id.into(),
            card_type: card_type.into(),
            location_id,
            code,
            open_id,
            timestamp,
            nonce_str: nonce_str.into(),
            signature: signature.into(),
        }
    }
}
