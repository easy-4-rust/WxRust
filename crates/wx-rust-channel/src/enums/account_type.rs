//! 视频号小店 枚举（对应 Java `AccountType`）。

/// AccountType（对应 Java `me.chanjar.weixin.channel.enums.AccountType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccountType {
    /// 对公银行账户
    AccountTypeBusiness,
    /// 经营者个人银行卡
    AccountTypePrivate,
}

impl AccountType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            AccountType::AccountTypeBusiness => "ACCOUNT_TYPE_BUSINESS",
            AccountType::AccountTypePrivate => "ACCOUNT_TYPE_PRIVATE",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            AccountType::AccountTypeBusiness => "对公银行账户",
            AccountType::AccountTypePrivate => "经营者个人银行卡",
        }
    }
}
