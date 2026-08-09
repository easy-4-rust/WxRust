//! 公众号二维码服务。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpQrcodeService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::result::WxMpQrCodeTicket;

/// 二维码服务。
#[async_trait]
pub trait WxMpQrcodeService: Send + Sync {
    /// 创建二维码 ticket。
    ///
    /// # 参数
    /// - `action_name`：二维码类型（如 `QR_SCENE`/`QR_STR_SCENE` 等）
    /// - `scene_str`：场景值（字符串形式）
    ///
    /// # 返回
    /// 二维码 ticket（含 url）。
    async fn qrcode_create_ticket(
        &self,
        action_name: &str,
        scene_str: &str,
    ) -> Result<WxMpQrCodeTicket, WxErrorException>;
}
