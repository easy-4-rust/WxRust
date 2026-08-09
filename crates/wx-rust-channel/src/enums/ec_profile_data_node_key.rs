//! 视频号小店 枚举（对应 Java `EcProfileDataNodeKey`）。

/// EcProfileDataNodeKey（对应 Java `me.chanjar.weixin.channel.enums.EcProfileDataNodeKey`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EcProfileDataNodeKey {
    /// 性别分布
    Sex,
    /// 年龄分布
    Age,
    /// 省份分布
    Province,
    /// 城市分布
    City,
    /// 关注关系分布
    Follow,
    /// 策略人群分布
    Cate,
    /// 商品类目偏好分布
    EcomUserLevel,
    /// 平均客单价分布
    GmvPerCnt,
}

impl EcProfileDataNodeKey {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            EcProfileDataNodeKey::Sex => "sex_distribution",
            EcProfileDataNodeKey::Age => "age_distribution",
            EcProfileDataNodeKey::Province => "province_distribution",
            EcProfileDataNodeKey::City => "city_distribution",
            EcProfileDataNodeKey::Follow => "follow_distribution",
            EcProfileDataNodeKey::Cate => "cate_distribution",
            EcProfileDataNodeKey::EcomUserLevel => "ecom_user_level_distribution",
            EcProfileDataNodeKey::GmvPerCnt => "gmv_per_cnt_distribution",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            EcProfileDataNodeKey::Sex => "性别分布",
            EcProfileDataNodeKey::Age => "年龄分布",
            EcProfileDataNodeKey::Province => "省份分布",
            EcProfileDataNodeKey::City => "城市分布",
            EcProfileDataNodeKey::Follow => "关注关系分布",
            EcProfileDataNodeKey::Cate => "策略人群分布",
            EcProfileDataNodeKey::EcomUserLevel => "商品类目偏好分布",
            EcProfileDataNodeKey::GmvPerCnt => "平均客单价分布",
        }
    }
}
