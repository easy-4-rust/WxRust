//! 视频号小店 枚举（对应 Java `SpuStatus`）。

/// SpuStatus（对应 Java `me.chanjar.weixin.channel.enums.SpuStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpuStatus {
    /// 0 初始值
    Init,
    /// 5 上架
    Up,
    /// 6 回收站
    Trash,
    /// 9 彻底删除，商品无法再进行任何操作
    Delete,
    /// 11 自主下架
    Down,
    /// 13 违规下架/风控系统下架
    SystemDown,
    /// 14 保证金不足下架
    DepositInsufficient,
    /// 15 品牌过期下架
    BrandExpired,
    /// 20 商品被封禁
    Ban,
}

impl SpuStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            SpuStatus::Init => 0,
            SpuStatus::Up => 5,
            SpuStatus::Trash => 6,
            SpuStatus::Delete => 9,
            SpuStatus::Down => 11,
            SpuStatus::SystemDown => 13,
            SpuStatus::DepositInsufficient => 14,
            SpuStatus::BrandExpired => 15,
            SpuStatus::Ban => 20,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            SpuStatus::Init => "未上架",
            SpuStatus::Up => "上架",
            SpuStatus::Trash => "回收站",
            SpuStatus::Delete => "彻底删除",
            SpuStatus::Down => "自主下架",
            SpuStatus::SystemDown => "违规下架/风控系统下架",
            SpuStatus::DepositInsufficient => "保证金不足下架",
            SpuStatus::BrandExpired => "品牌过期下架",
            SpuStatus::Ban => "商品被封禁",
        }
    }
}
