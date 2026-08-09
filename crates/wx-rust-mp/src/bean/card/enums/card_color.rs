//! 卡券枚举：CardColor。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardColor`。

/// CardColor。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardColor {
    /// #63b359
    #[default]
    Color010,
    /// #2c9f67
    Color020,
    /// #509fc9
    Color030,
    /// #5885cf
    Color040,
    /// #9062c0
    Color050,
    /// #d09a45
    Color060,
    /// #e4b138
    Color070,
    /// #ee903c
    Color080,
    /// #f08500
    Color081,
    /// #a9d92d
    Color082,
    /// #dd6549
    Color090,
    /// #cc463d
    Color100,
    /// #cf3e36
    Color101,
    /// #5E6671
    Color102,
}

impl CardColor {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 14] = [
        Self::Color010,
        Self::Color020,
        Self::Color030,
        Self::Color040,
        Self::Color050,
        Self::Color060,
        Self::Color070,
        Self::Color080,
        Self::Color081,
        Self::Color082,
        Self::Color090,
        Self::Color100,
        Self::Color101,
        Self::Color102,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::Color010 => "#63b359",
            Self::Color020 => "#2c9f67",
            Self::Color030 => "#509fc9",
            Self::Color040 => "#5885cf",
            Self::Color050 => "#9062c0",
            Self::Color060 => "#d09a45",
            Self::Color070 => "#e4b138",
            Self::Color080 => "#ee903c",
            Self::Color081 => "#f08500",
            Self::Color082 => "#a9d92d",
            Self::Color090 => "#dd6549",
            Self::Color100 => "#cc463d",
            Self::Color101 => "#cf3e36",
            Self::Color102 => "#5E6671",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
