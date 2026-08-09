//! 设备被动回复消息。
//!
//! 对应 Java `WxMpXmlOutDeviceMessage`：微信硬件平台回复。

use super::wx_mp_xml_out_message::{WxMpXmlOutMessage, push_cdata_field};

/// 设备消息（`MsgType = device_text`）。
#[derive(Debug, Clone, Default)]
pub struct WxMpXmlOutDeviceMessage {
    /// 公共字段（组合父类语义）。
    pub base: WxMpXmlOutMessage,
    /// 设备类型。
    pub device_type: Option<String>,
    /// 设备 id。
    pub device_id: Option<String>,
    /// 会话 id。
    pub session_id: Option<String>,
    /// 消息内容。
    pub content: Option<String>,
}

impl WxMpXmlOutDeviceMessage {
    /// 构造设备消息。
    pub fn new() -> Self {
        Self {
            base: WxMpXmlOutMessage {
                msg_type: Some("device_text".to_string()),
                ..Default::default()
            },
            device_type: None,
            device_id: None,
            session_id: None,
            content: None,
        }
    }

    /// 转换成 xml 格式。
    pub fn to_xml(&self) -> String {
        let mut body = String::new();
        push_cdata_field(&mut body, "DeviceType", self.device_type.as_deref());
        push_cdata_field(&mut body, "DeviceID", self.device_id.as_deref());
        push_cdata_field(&mut body, "SessionID", self.session_id.as_deref());
        push_cdata_field(&mut body, "Content", self.content.as_deref());
        self.base.to_xml(&body)
    }
}
