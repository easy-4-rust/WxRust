//! 视频号小店 枚举（对应 Java `AfterSaleType`）。

/// AfterSaleType（对应 Java `me.chanjar.weixin.channel.enums.AfterSaleType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AfterSaleType {
    /// 1 仅退款
    RefundOnly,
    /// 2 退货退款
    RefundGoods,
}

impl AfterSaleType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            AfterSaleType::RefundOnly => "REFUND",
            AfterSaleType::RefundGoods => "RETURN",
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            AfterSaleType::RefundOnly => "仅退款",
            AfterSaleType::RefundGoods => "退货退款",
        }
    }
}
