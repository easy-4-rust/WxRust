//! 微信小店 枚举（对应 Java `MessageType`）。

/// MessageType（对应 Java `com.binarywang.wxjava.store.enums.MessageType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessageType {
    Event,
}

impl MessageType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            MessageType::Event => "event",
        }
    }
}
