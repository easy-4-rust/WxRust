//! 微信类型枚举。
//!
//! 对应 Java `me.chanjar.weixin.common.enums.WxType`。

/// 微信类型枚举，标识当前请求/响应所属的微信平台。
///
/// 用于错误码翻译（不同平台错误码表不同）、日志与通用执行器的平台分发。
///
/// # 变体
/// - `Cp`：企业微信
/// - `Mp`：微信公众号
/// - `MiniApp`：微信小程序
/// - `Open`：微信开放平台
/// - `Pay`：微信支付
/// - `Channel`：微信视频号
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WxType {
    /// 企业微信
    Cp,
    /// 微信公众号
    Mp,
    /// 微信小程序
    MiniApp,
    /// 微信开放平台
    Open,
    /// 微信支付
    Pay,
    /// 微信视频号
    Channel,
}

impl WxType {
    /// 返回枚举的字符串表示（与 Java `name()` 对齐，`MiniApp` 保持驼峰）。
    pub fn name(self) -> &'static str {
        match self {
            WxType::Cp => "CP",
            WxType::Mp => "MP",
            WxType::MiniApp => "MiniApp",
            WxType::Open => "Open",
            WxType::Pay => "Pay",
            WxType::Channel => "Channel",
        }
    }
}

impl std::fmt::Display for WxType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
