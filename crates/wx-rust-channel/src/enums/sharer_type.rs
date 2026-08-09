//! 视频号小店 枚举（对应 Java `SharerType`）。

/// SharerType（对应 Java `me.chanjar.weixin.channel.enums.SharerType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SharerType {
    /// 0 普通分享员
    Normal,
    /// 1 企业分享员
    Enterprise,
}

impl SharerType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            SharerType::Normal => 0,
            SharerType::Enterprise => 1,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            SharerType::Normal => "普通分享员",
            SharerType::Enterprise => "企业分享员",
        }
    }
}
