//! 换取二维码的 Ticket。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpQrCodeTicket`。

use serde::{Deserialize, Serialize};

/// 换取二维码的 Ticket。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpQrCodeTicket {
    /// 获取的二维码 ticket。
    #[serde(default)]
    pub ticket: String,
    /// 二维码有效时间（秒）；-1 表示永久。
    #[serde(default = "default_expire_seconds")]
    pub expire_seconds: i32,
    /// 二维码图片解析后的地址。
    #[serde(default)]
    pub url: String,
}

/// 默认有效时间 -1（永久）。
fn default_expire_seconds() -> i32 {
    -1
}

impl WxMpQrCodeTicket {
    /// 从 JSON 构建（对应 Java `fromJson`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("二维码 Ticket 解析失败: {e}"))
    }
}
