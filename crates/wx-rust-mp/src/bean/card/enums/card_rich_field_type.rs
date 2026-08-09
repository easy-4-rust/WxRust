//! 卡券枚举：CardRichFieldType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardRichFieldType`。

/// CardRichFieldType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardRichFieldType {
    /// 自定义单选
    #[default]
    FormFieldRadio,
    /// 自定义选择项
    FormFieldSelect,
    /// 自定义多选
    FormFieldCheckBox,
}

impl CardRichFieldType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 3] = [
        Self::FormFieldRadio,
        Self::FormFieldSelect,
        Self::FormFieldCheckBox,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::FormFieldRadio => "自定义单选",
            Self::FormFieldSelect => "自定义选择项",
            Self::FormFieldCheckBox => "自定义多选",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
