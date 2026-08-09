//! 客服转接被动回复消息。
//!
//! 对应 Java `WxMpXmlOutTransferKefuMessage`。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, transfer_kefu_to_xml};

/// 客服转接消息（`MsgType = transfer_customer_service`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutTransferKefuMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 指定会话客服的帐号。
    pub kf_account: Option<String>,
}

impl WxMpXmlOutTransferKefuMessage {
    /// 构造客服转接消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("transfer_customer_service".to_string()),
                ..Default::default()
            },
            kf_account: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        transfer_kefu_to_xml(self)
    }
}
