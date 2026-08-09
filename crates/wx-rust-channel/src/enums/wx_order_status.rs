//! 视频号小店 枚举（对应 Java `WxOrderStatus`）。

/// WxOrderStatus（对应 Java `me.chanjar.weixin.channel.enums.WxOrderStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WxOrderStatus {
    /// 10 待付款
    Unpaid,
    /// 20 待发货（已付款/用户已付尾款）
    Paid,
    /// 21 部分发货
    PartDelivery,
    /// 30 待收货
    Delivery,
    /// 100 完成
    Completed,
    /// 190 商品超卖商家取消订单
    UnpaidCancel,
    /// 200 全部商品售后之后，订单取消
    AllAfterSale,
    /// 250 用户主动取消/待付款超时取消/商家取消
    Cancel,
}

impl WxOrderStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            WxOrderStatus::Unpaid => 10,
            WxOrderStatus::Paid => 20,
            WxOrderStatus::PartDelivery => 21,
            WxOrderStatus::Delivery => 30,
            WxOrderStatus::Completed => 100,
            WxOrderStatus::UnpaidCancel => 190,
            WxOrderStatus::AllAfterSale => 200,
            WxOrderStatus::Cancel => 250,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            WxOrderStatus::Unpaid => "待付款",
            WxOrderStatus::Paid => "待发货",
            WxOrderStatus::PartDelivery => "部分发货",
            WxOrderStatus::Delivery => "待收货",
            WxOrderStatus::Completed => "已完成",
            WxOrderStatus::UnpaidCancel => "已取消",
            WxOrderStatus::AllAfterSale => "已取消",
            WxOrderStatus::Cancel => "已取消",
        }
    }

    /// 获取状态中文（对应 Java `getStatusStr(Integer)`；未知返回数字字符串）。
    pub fn get_status_str(key: Option<i32>) -> String {
        match key {
            Some(k) => {
                for s in [
                    WxOrderStatus::Unpaid,
                    WxOrderStatus::Paid,
                    WxOrderStatus::PartDelivery,
                    WxOrderStatus::Delivery,
                    WxOrderStatus::Completed,
                    WxOrderStatus::UnpaidCancel,
                    WxOrderStatus::AllAfterSale,
                    WxOrderStatus::Cancel,
                ] {
                    if s.key() == k {
                        return s.val().to_string();
                    }
                }
                k.to_string()
            }
            None => "未知".to_string(),
        }
    }

    /// 判断订单是否处于取消状态（对应 Java `isCancel(Integer)`）。
    pub fn is_cancel(key: Option<i32>) -> bool {
        let Some(key) = key else { return false };
        matches!(key, 190 | 200 | 250)
    }
}
