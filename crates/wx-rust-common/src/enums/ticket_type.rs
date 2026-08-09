//! ticket 类型枚举。
//!
//! 对应 Java `me.chanjar.weixin.common.enums.TicketType`。

/// 微信 jsapi ticket 类型枚举，携带调用微信接口时使用的 ticket 值。
///
/// 用于 jsapi 签名、卡券等场景的票据缓存（公众号/企业微信等模块共用）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TicketType {
    /// jsapi，用于 JS-SDK 签名
    Jsapi,
    /// sdk，值为 `"2"`
    Sdk,
    /// 微信卡券
    WxCard,
}

impl TicketType {
    /// 返回该 ticket 类型对应的接口参数值。
    ///
    /// - `Jsapi` → `"jsapi"`
    /// - `Sdk` → `"2"`
    /// - `WxCard` → `"wx_card"`
    pub fn value(self) -> &'static str {
        match self {
            TicketType::Jsapi => "jsapi",
            TicketType::Sdk => "2",
            TicketType::WxCard => "wx_card",
        }
    }
}
