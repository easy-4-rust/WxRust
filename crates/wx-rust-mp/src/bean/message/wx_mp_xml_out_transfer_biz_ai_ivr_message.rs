//! 转接 AI 回复被动回复消息。
//!
//! 对应 Java `WxMpXmlOutTransferBizAiIvrMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, transfer_biz_ai_ivr_to_xml};

/// 转接 AI 回复消息（`MsgType = transfer_biz_ai_ivr`，无额外字段）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutTransferBizAiIvrMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
}

impl WxMpXmlOutTransferBizAiIvrMessage {
    /// 构造转接 AI 回复消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("transfer_biz_ai_ivr".to_string()),
                ..Default::default()
            },
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        transfer_biz_ai_ivr_to_xml(self)
    }
}
