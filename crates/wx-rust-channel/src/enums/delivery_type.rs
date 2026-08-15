//! 视频号小店 枚举（对应 Java `DeliveryType`）。

/// DeliveryType（对应 Java `me.chanjar.weixin.channel.enums.DeliveryType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeliveryType {
    /// 1 自寄快递
    SelfDelivery,
    /// 2 在线签约快递单
    OnlineDelivery,
    /// 3 虚拟商品无需物流发货
    VirtualDelivery,
    /// 4 在线快递散单
    OnlineDeliveryScatter,
}

impl DeliveryType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            DeliveryType::SelfDelivery => 1,
            DeliveryType::OnlineDelivery => 2,
            DeliveryType::VirtualDelivery => 3,
            DeliveryType::OnlineDeliveryScatter => 4,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            DeliveryType::SelfDelivery => "自寄快递",
            DeliveryType::OnlineDelivery => "在线签约快递单",
            DeliveryType::VirtualDelivery => "虚拟商品无需物流发货",
            DeliveryType::OnlineDeliveryScatter => "在线快递散单",
        }
    }

    /// 按 key 查找（对应 Java `getDeliveryType(Integer)`；找不到返回 `None`）。
    pub fn get_delivery_type(key: i32) -> Option<Self> {
        [
            DeliveryType::SelfDelivery,
            DeliveryType::OnlineDelivery,
            DeliveryType::VirtualDelivery,
            DeliveryType::OnlineDeliveryScatter,
        ]
        .into_iter()
        .find(|&v| v.key() == key)
    }
}
