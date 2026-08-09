//! 卡券枚举：BusinessServiceType。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.card.enums.BusinessServiceType`。

/// BusinessServiceType。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessServiceType {
    /// 外卖服务
    #[default]
    BizServiceDeliver,
    /// 停车位
    BizServiceFreePark,
    /// 可带宠物
    BizServiceWithPet,
    /// 可带宠物
    BizServiceFreeWifi,
}

impl BusinessServiceType {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 4] = [
        Self::BizServiceDeliver,
        Self::BizServiceFreePark,
        Self::BizServiceWithPet,
        Self::BizServiceFreeWifi,
    ];

    /// 枚举值（对应 Java 构造参数，如描述/颜色）。
    pub fn value(self) -> &'static str {
        match self {
            Self::BizServiceDeliver => "外卖服务",
            Self::BizServiceFreePark => "停车位",
            Self::BizServiceWithPet => "可带宠物",
            Self::BizServiceFreeWifi => "可带宠物",
        }
    }

    /// 按枚举值查找。
    pub fn find_by_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.value() == value)
    }
}
