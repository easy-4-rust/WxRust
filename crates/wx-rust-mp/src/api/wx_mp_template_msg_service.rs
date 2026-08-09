//! 公众号模板消息服务。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpTemplateMsgService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::template::WxMpTemplateMessage;

/// 模板消息服务。
#[async_trait]
pub trait WxMpTemplateMsgService: Send + Sync {
    /// 发送模板消息。
    ///
    /// # 参数
    /// - `message`：模板消息
    ///
    /// # 返回
    /// 接口响应（`{"errcode":0,"msgid":...}` 的原始 JSON）。
    async fn send_template_msg(
        &self,
        message: &WxMpTemplateMessage,
    ) -> Result<String, WxErrorException>;
}
