//! 视频号小店 枚举（对应 Java `AfterSaleStatus`）。

/// AfterSaleStatus（对应 Java `me.chanjar.weixin.channel.enums.AfterSaleStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AfterSaleStatus {
    /// 用户取消申请
    UserCanceld,
    /// 商家受理中
    MerchantProcessing,
    /// 商家拒绝退款
    MerchantRejectRefund,
    /// 商家拒绝退货退款
    MerchantRejectReturn,
    /// 待买家退货
    UserWaitReturn,
    /// 7 售后单关闭
    ReturnClosed,
    /// 8 待商家收货
    MerchantWaitReceipt,
    /// 商家逾期未退款
    MerchantOverdueRefund,
    /// 退款完成
    MerchantRefundSuccess,
    /// 退货退款完成
    MerchantReturnSuccess,
    /// 11 平台退款中
    PlatformRefunding,
    /// 25 平台退款失败
    PlatformRefundFail,
    /// 待用户确认
    UserWaitConfirm,
    /// 商家打款失败，客服关闭售后
    MerchantRefundRetryFail,
    /// 售后关闭
    MerchantFail,
    /// 待用户处理商家协商
    UserWaitConfirmUpdate,
    /// 待用户处理商家代发起的售后申请
    UserWaitHandleMerchantAfterSale,
}

impl AfterSaleStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> &'static str {
        match self {
            AfterSaleStatus::UserCanceld => "USER_CANCELD",
            AfterSaleStatus::MerchantProcessing => "MERCHANT_PROCESSING",
            AfterSaleStatus::MerchantRejectRefund => "MERCHANT_REJECT_REFUND",
            AfterSaleStatus::MerchantRejectReturn => "MERCHANT_REJECT_RETURN",
            AfterSaleStatus::UserWaitReturn => "USER_WAIT_RETURN",
            AfterSaleStatus::ReturnClosed => "RETURN_CLOSED",
            AfterSaleStatus::MerchantWaitReceipt => "MERCHANT_WAIT_RECEIPT",
            AfterSaleStatus::MerchantOverdueRefund => "MERCHANT_OVERDUE_REFUND",
            AfterSaleStatus::MerchantRefundSuccess => "MERCHANT_REFUND_SUCCESS",
            AfterSaleStatus::MerchantReturnSuccess => "MERCHANT_RETURN_SUCCESS",
            AfterSaleStatus::PlatformRefunding => "PLATFORM_REFUNDING",
            AfterSaleStatus::PlatformRefundFail => "PLATFORM_REFUND_FAIL",
            AfterSaleStatus::UserWaitConfirm => "USER_WAIT_CONFIRM",
            AfterSaleStatus::MerchantRefundRetryFail => "MERCHANT_REFUND_RETRY_FAIL",
            AfterSaleStatus::MerchantFail => "MERCHANT_FAIL",
            AfterSaleStatus::UserWaitConfirmUpdate => "USER_WAIT_CONFIRM_UPDATE",
            AfterSaleStatus::UserWaitHandleMerchantAfterSale => {
                "USER_WAIT_HANDLE_MERCHANT_AFTER_SALE"
            }
        }
    }

    /// 枚举中文说明（对应 Java `getValue()`）。
    pub fn value(&self) -> &'static str {
        match self {
            AfterSaleStatus::UserCanceld => "用户取消申请",
            AfterSaleStatus::MerchantProcessing => "商家受理中",
            AfterSaleStatus::MerchantRejectRefund => "商家拒绝退款",
            AfterSaleStatus::MerchantRejectReturn => "商家拒绝退货退款",
            AfterSaleStatus::UserWaitReturn => "待买家退货",
            AfterSaleStatus::ReturnClosed => "退货退款关闭",
            AfterSaleStatus::MerchantWaitReceipt => "待商家收货",
            AfterSaleStatus::MerchantOverdueRefund => "商家逾期未退款",
            AfterSaleStatus::MerchantRefundSuccess => "退款完成",
            AfterSaleStatus::MerchantReturnSuccess => "退货退款完成",
            AfterSaleStatus::PlatformRefunding => "平台退款中",
            AfterSaleStatus::PlatformRefundFail => "平台退款失败",
            AfterSaleStatus::UserWaitConfirm => "待用户确认",
            AfterSaleStatus::MerchantRefundRetryFail => "商家打款失败，客服关闭售后",
            AfterSaleStatus::MerchantFail => "售后关闭",
            AfterSaleStatus::UserWaitConfirmUpdate => "待用户处理商家协商",
            AfterSaleStatus::UserWaitHandleMerchantAfterSale => "待用户处理商家代发起的售后申请",
        }
    }
}
