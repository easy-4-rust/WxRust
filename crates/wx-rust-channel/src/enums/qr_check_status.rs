//! 视频号小店 枚举（对应 Java `QrCheckStatus`）。

/// QrCheckStatus（对应 Java `me.chanjar.weixin.channel.enums.QrCheckStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QrCheckStatus {
    /// 0 未扫码
    NotScan,
    /// 1 已确认
    Confirmed,
    /// 2 已取消
    Cancel,
    /// 3 已失效
    Invalid,
    /// 4 已扫码
    Scan,
}

impl QrCheckStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            QrCheckStatus::NotScan => 0,
            QrCheckStatus::Confirmed => 1,
            QrCheckStatus::Cancel => 2,
            QrCheckStatus::Invalid => 3,
            QrCheckStatus::Scan => 4,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            QrCheckStatus::NotScan => "未扫码",
            QrCheckStatus::Confirmed => "已确认",
            QrCheckStatus::Cancel => "已取消",
            QrCheckStatus::Invalid => "已失效",
            QrCheckStatus::Scan => "已扫码",
        }
    }
}
