//! 卡券枚举：CardCodeType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardCodeType`。

/// CardCodeType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardCodeType {
    /// 文本
    #[default]
    CodeTypeText,
    /// 不显示任何码型
    CodeTypeNone,
    /// 仅显示一维码
    CodeTypeOnlyBarcode,
    /// 仅显示二维码
    CodeTypeOnlyQrcode,
    /// 一维码
    CodeTypeBarcode,
    /// 二维码
    CodeTypeQrcode,
}

impl CardCodeType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 6] = [
        Self::CodeTypeText,
        Self::CodeTypeNone,
        Self::CodeTypeOnlyBarcode,
        Self::CodeTypeOnlyQrcode,
        Self::CodeTypeBarcode,
        Self::CodeTypeQrcode,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::CodeTypeText => "文本",
            Self::CodeTypeNone => "不显示任何码型",
            Self::CodeTypeOnlyBarcode => "仅显示一维码",
            Self::CodeTypeOnlyQrcode => "仅显示二维码",
            Self::CodeTypeBarcode => "一维码",
            Self::CodeTypeQrcode => "二维码",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
