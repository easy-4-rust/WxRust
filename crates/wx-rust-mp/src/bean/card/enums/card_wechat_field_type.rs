//! 卡券枚举：CardWechatFieldType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardWechatFieldType`。

/// CardWechatFieldType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardWechatFieldType {
    /// 手机号
    #[default]
    UserFormInfoFlagMobile,
    /// 性别
    UserFormInfoFlagSex,
    /// 姓名
    UserFormInfoFlagName,
    /// 生日
    UserFormInfoFlagBirthday,
    /// 身份证
    UserFormInfoFlagIdcard,
    /// 邮箱
    UserFormInfoFlagEmail,
    /// 详细地址
    UserFormInfoFlagLocation,
    /// 教育背景
    UserFormInfoFlagEducationBackground,
    /// 行业
    UserFormInfoFlagIndustry,
    /// 收入
    UserFormInfoFlagIncome,
    /// 兴趣爱好
    UserFormInfoFlagHabit,
}

impl CardWechatFieldType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 11] = [
        Self::UserFormInfoFlagMobile,
        Self::UserFormInfoFlagSex,
        Self::UserFormInfoFlagName,
        Self::UserFormInfoFlagBirthday,
        Self::UserFormInfoFlagIdcard,
        Self::UserFormInfoFlagEmail,
        Self::UserFormInfoFlagLocation,
        Self::UserFormInfoFlagEducationBackground,
        Self::UserFormInfoFlagIndustry,
        Self::UserFormInfoFlagIncome,
        Self::UserFormInfoFlagHabit,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::UserFormInfoFlagMobile => "手机号",
            Self::UserFormInfoFlagSex => "性别",
            Self::UserFormInfoFlagName => "姓名",
            Self::UserFormInfoFlagBirthday => "生日",
            Self::UserFormInfoFlagIdcard => "身份证",
            Self::UserFormInfoFlagEmail => "邮箱",
            Self::UserFormInfoFlagLocation => "详细地址",
            Self::UserFormInfoFlagEducationBackground => "教育背景",
            Self::UserFormInfoFlagIndustry => "行业",
            Self::UserFormInfoFlagIncome => "收入",
            Self::UserFormInfoFlagHabit => "兴趣爱好",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
