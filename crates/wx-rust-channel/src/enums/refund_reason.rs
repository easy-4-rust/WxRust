//! 视频号小店 枚举（对应 Java `RefundReason`）。

/// RefundReason（对应 Java `me.chanjar.weixin.channel.enums.RefundReason`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RefundReason {
    /// 1 商家通过店铺管理页或者小助手发起退款
    MerchantInitiatedRefund,
    /// 2 退货退款场景，商家同意买家未上传物流单号情况下确认收货并退款，该场景限于订单无运费险
    MerchantAgreesNoTrackingRefund,
    /// 3 商家通过后台api发起退款
    MerchantApiInitiatedRefund,
    /// 4 未发货售后平台自动同意
    PreShipmentAutomaticRefund,
    /// 5 平台介入纠纷退款
    PlatformIntervenedDisputeRefund,
    /// 6 特殊场景下平台强制退款
    PlatformForcedRefund,
    /// 7 退货退款场景，买家同意没有上传物流单号情况下，商家确认收货并退款，该场景限于订单包含运费险，并无法理赔
    BuyerAgreesNoTrackingRefund,
    /// 8 商家发货超时，平台退款
    LateShipmentPlatformRefund,
    /// 9 商家处理买家售后申请超时，平台自动同意退款
    MerchantOverdueAutoRefund,
    /// 10 用户确认收货超时，平台退款
    BuyerOverdueAutoRefund,
    /// 11 商家确认收货超时，平台退款
    MerchantOverdueConfirmationRefund,
}

impl RefundReason {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            RefundReason::MerchantInitiatedRefund => 1,
            RefundReason::MerchantAgreesNoTrackingRefund => 2,
            RefundReason::MerchantApiInitiatedRefund => 3,
            RefundReason::PreShipmentAutomaticRefund => 4,
            RefundReason::PlatformIntervenedDisputeRefund => 5,
            RefundReason::PlatformForcedRefund => 6,
            RefundReason::BuyerAgreesNoTrackingRefund => 7,
            RefundReason::LateShipmentPlatformRefund => 8,
            RefundReason::MerchantOverdueAutoRefund => 9,
            RefundReason::BuyerOverdueAutoRefund => 10,
            RefundReason::MerchantOverdueConfirmationRefund => 11,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            RefundReason::MerchantInitiatedRefund => "商家通过店铺管理页或者小助手发起退款",
            RefundReason::MerchantAgreesNoTrackingRefund => {
                "退货退款场景，商家同意买家未上传物流单号情况下确认收货并退款，该场景限于订单无运费险"
            }
            RefundReason::MerchantApiInitiatedRefund => "商家通过后台api发起退款",
            RefundReason::PreShipmentAutomaticRefund => "未发货售后平台自动同意",
            RefundReason::PlatformIntervenedDisputeRefund => "平台介入纠纷退款",
            RefundReason::PlatformForcedRefund => "特殊场景下平台强制退款",
            RefundReason::BuyerAgreesNoTrackingRefund => {
                "退货退款场景，买家同意没有上传物流单号情况下，商家确认收货并退款，该场景限于订单包含运费险，并无法理赔"
            }
            RefundReason::LateShipmentPlatformRefund => "商家发货超时，平台退款",
            RefundReason::MerchantOverdueAutoRefund => "商家处理买家售后申请超时，平台自动同意退款",
            RefundReason::BuyerOverdueAutoRefund => "用户确认收货超时，平台退款",
            RefundReason::MerchantOverdueConfirmationRefund => "商家确认收货超时，平台退款",
        }
    }
}
