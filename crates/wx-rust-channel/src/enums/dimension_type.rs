//! 视频号小店 枚举（对应 Java `DimensionType`）。

/// DimensionType（对应 Java `me.chanjar.weixin.channel.enums.DimensionType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DimensionType {
    /// 一级渠道
    PrimaryChannel,
    /// 年龄段
    Age,
    /// 性别
    Sex,
    /// 关注关系
    Follow,
    /// 二级渠道
    SecondaryChannel,
    /// 策略人群
    Cate,
    /// 省级行政区
    Province,
    /// 地级行政区
    City,
    /// 消费者商品类目偏好
    EcomUserLevel,
    /// 客单价区间
    GmvPerCnt,
    /// // 关注关系 // / // FOLLOW(15, "关注关系"), / 流量类型（自然流量、直播加热、广告投流）
    Flow,
}

impl DimensionType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            DimensionType::PrimaryChannel => 1,
            DimensionType::Age => 2,
            DimensionType::Sex => 3,
            DimensionType::Follow => 5,
            DimensionType::SecondaryChannel => 7,
            DimensionType::Cate => 9,
            DimensionType::Province => 10,
            DimensionType::City => 11,
            DimensionType::EcomUserLevel => 12,
            DimensionType::GmvPerCnt => 13,
            DimensionType::Flow => 16,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            DimensionType::PrimaryChannel => "一级渠道",
            DimensionType::Age => "年龄段",
            DimensionType::Sex => "性别",
            DimensionType::Follow => "关注关系",
            DimensionType::SecondaryChannel => "二级渠道",
            DimensionType::Cate => "策略人群",
            DimensionType::Province => "省级行政区",
            DimensionType::City => "地级行政区",
            DimensionType::EcomUserLevel => "消费者商品类目偏好",
            DimensionType::GmvPerCnt => "客单价区间",
            DimensionType::Flow => "流量类型（自然流量、直播加热、广告投流）",
        }
    }
}
