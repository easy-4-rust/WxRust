//! 设备消息 builder。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.DeviceBuilder`：构建
//! 微信硬件平台回复消息。

use crate::bean::message::wx_mp_xml_out_device_message::WxMpXmlOutDeviceMessage;
use crate::bean::message::wx_mp_xml_out_message::WxMpXmlOutMessage;

/// 设备消息 builder。
#[derive(Debug, Default)]
pub struct DeviceBuilder {
    msg: WxMpXmlOutDeviceMessage,
}

impl DeviceBuilder {
    /// 构建设备消息 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置设备类型。
    pub fn device_type(mut self, device_type: impl Into<String>) -> Self {
        self.msg.device_type = Some(device_type.into());
        self
    }

    /// 设置设备 id。
    pub fn device_id(mut self, device_id: impl Into<String>) -> Self {
        self.msg.device_id = Some(device_id.into());
        self
    }

    /// 设置消息内容。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.msg.content = Some(content.into());
        self
    }

    /// 设置基础字段。
    pub fn from(mut self, base: WxMpXmlOutMessage) -> Self {
        self.msg.base = base;
        self
    }

    /// 构建消息。
    pub fn build(self) -> WxMpXmlOutDeviceMessage {
        self.msg
    }
}
