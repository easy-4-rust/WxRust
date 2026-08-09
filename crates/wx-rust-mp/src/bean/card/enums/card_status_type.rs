//! 卡券枚举：CardStatusType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.CardStatusType`。

/// CardStatusType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardStatusType {
    /// 待审核
    #[default]
    CardStatusNotVerify,
    /// 审核失败
    CardStatusVerifyFail,
    /// 通过审核
    CardStatusVerifyOk,
    /// 卡券被商户删除
    CardStatusDelete,
    /// 在公众平台投放过的卡券
    CardStatusDispatch,
}

impl CardStatusType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 5] = [
        Self::CardStatusNotVerify,
        Self::CardStatusVerifyFail,
        Self::CardStatusVerifyOk,
        Self::CardStatusDelete,
        Self::CardStatusDispatch,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::CardStatusNotVerify => "待审核",
            Self::CardStatusVerifyFail => "审核失败",
            Self::CardStatusVerifyOk => "通过审核",
            Self::CardStatusDelete => "卡券被商户删除",
            Self::CardStatusDispatch => "在公众平台投放过的卡券",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
