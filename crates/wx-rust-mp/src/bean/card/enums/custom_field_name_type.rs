//! 卡券枚举：CustomFieldNameType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CustomFieldNameType`。

/// CustomFieldNameType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomFieldNameType {
    /// 等级
    #[default]
    FieldNameTypeLevel,
    /// 优惠券
    FieldNameTypeCoupon,
    /// 印花
    FieldNameTypeStamp,
    /// 折扣
    FieldNameTypeDiscount,
    /// 成就
    FieldNameTypeAchievemen,
    /// 里程
    FieldNameTypeMileage,
    /// 集点
    FieldNameTypeSetPoints,
    /// 次数
    FieldNameTypeTims,
}

impl CustomFieldNameType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 8] = [
        Self::FieldNameTypeLevel,
        Self::FieldNameTypeCoupon,
        Self::FieldNameTypeStamp,
        Self::FieldNameTypeDiscount,
        Self::FieldNameTypeAchievemen,
        Self::FieldNameTypeMileage,
        Self::FieldNameTypeSetPoints,
        Self::FieldNameTypeTims,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::FieldNameTypeLevel => "等级",
            Self::FieldNameTypeCoupon => "优惠券",
            Self::FieldNameTypeStamp => "印花",
            Self::FieldNameTypeDiscount => "折扣",
            Self::FieldNameTypeAchievemen => "成就",
            Self::FieldNameTypeMileage => "里程",
            Self::FieldNameTypeSetPoints => "集点",
            Self::FieldNameTypeTims => "次数",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
