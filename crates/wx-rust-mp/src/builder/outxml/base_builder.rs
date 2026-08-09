//! Builder 基类。
//!
//! 对应 Java `me.chanjar.weixin.mp.builder.outxml.BaseBuilder`。

use crate::bean::message::WxMpXmlOutMessage;

/// 被动回复消息 builder 基类：持有公共字段。
#[derive(Debug, Clone, Default)]
pub struct BaseBuilder {
    /// 接收方帐号。
    pub(crate) to_user_name: Option<String>,
    /// 开发者微信号。
    pub(crate) from_user_name: Option<String>,
}

impl BaseBuilder {
    /// 构建空 builder。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置接收方帐号（对应 Java `toUser`）。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user_name = Some(to_user.into());
        self
    }

    /// 设置开发者微信号（对应 Java `fromUser`）。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.from_user_name = Some(from_user.into());
        self
    }
}

/// 将 builder 公共字段写入消息并设置创建时间（对应 Java `BaseBuilder.setCommon`）。
pub(crate) fn set_common(m: &mut WxMpXmlOutMessage, builder: BaseBuilder) {
    m.to_user_name = builder.to_user_name;
    m.from_user_name = builder.from_user_name;
    m.create_time = Some(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0),
    );
}
