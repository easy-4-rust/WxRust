//! 被动回复消息 builder 基类。
//!
//! 对应 Java `me.chanjar.weixin.cp.bean.outxmlbuilder.BaseBuilder`：持有
//! 公共字段（toUserName/fromUserName），`setCommon` 时写入创建时间
//! （当前时间戳秒，对应 Java `System.currentTimeMillis() / 1000L`）。

use crate::bean::message::WxCpXmlOutMessage;

/// 被动回复消息 builder 基类。
#[derive(Debug, Clone, Default)]
pub struct BaseBuilder {
    /// 接收方帐号。
    pub(crate) to_user_name: Option<String>,
    /// 开发者微信号。
    pub(crate) from_user_name: Option<String>,
}

impl BaseBuilder {
    /// 设置接收方帐号（对应 Java `toUser(String)`）。
    pub fn to_user(mut self, to_user: impl Into<String>) -> Self {
        self.to_user_name = Some(to_user.into());
        self
    }

    /// 设置开发者微信号（对应 Java `fromUser(String)`）。
    pub fn from_user(mut self, from_user: impl Into<String>) -> Self {
        self.from_user_name = Some(from_user.into());
        self
    }

    /// 将公共字段写入消息并设置创建时间（对应 Java `BaseBuilder.setCommon`）。
    pub(crate) fn set_common(self, m: &mut WxCpXmlOutMessage) {
        m.to_user_name = self.to_user_name;
        m.from_user_name = self.from_user_name;
        m.create_time = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs() as i64)
                .unwrap_or(0),
        );
    }
}
