//! AI 开放接口语言类型。
//!
//! 对应 Java `me.chanjar.weixin.mp.enums.AiLangType`。

/// AI 开放接口语言类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiLangType {
    /// 中文。
    ZhCn,
    /// 英文。
    EnUs,
}

impl AiLangType {
    /// 语言代码。
    pub fn code(self) -> &'static str {
        match self {
            Self::ZhCn => "zh_CN",
            Self::EnUs => "en_US",
        }
    }
}
