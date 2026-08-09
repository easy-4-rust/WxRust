//! 卡券枚举：CardFieldType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardFieldType`。

/// CardFieldType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardFieldType {
    /// 微信选项
    #[default]
    CommonField,
    /// 自定义选项
    CustomField,
    /// 自定义富文本类型
    RichField,
}

impl CardFieldType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 3] = [Self::CommonField, Self::CustomField, Self::RichField];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::CommonField => "微信选项",
            Self::CustomField => "自定义选项",
            Self::RichField => "自定义富文本类型",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
