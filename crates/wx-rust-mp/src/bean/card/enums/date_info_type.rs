//! 卡券枚举：DateInfoType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.DateInfoType`。

/// DateInfoType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateInfoType {
    /// 永久有效类型
    #[default]
    DateTypePermanent,
    /// 固定日期
    DateTypeFixTimeRange,
    /// 固定时长
    DateTypeFixTerm,
}

impl DateInfoType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 3] = [
        Self::DateTypePermanent,
        Self::DateTypeFixTimeRange,
        Self::DateTypeFixTerm,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::DateTypePermanent => "永久有效类型",
            Self::DateTypeFixTimeRange => "固定日期",
            Self::DateTypeFixTerm => "固定时长",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
