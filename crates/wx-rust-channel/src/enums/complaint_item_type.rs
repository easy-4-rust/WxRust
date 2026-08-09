//! 视频号小店 枚举（对应 Java `ComplaintItemType`）。

/// ComplaintItemType（对应 Java `me.chanjar.weixin.channel.enums.ComplaintItemType`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComplaintItemType {
    /// 1 申请平台介入
    ApplyPlatformIntervention,
    /// 2 用户留言
    UserMessage,
    /// 3 商家留言
    MerchantMessage,
    /// 4 提交投诉成功
    SubmitComplaintSuccess,
    /// 5 投诉已取消
    ComplaintCancelled,
    /// 6 商家已超时
    MerchantTimeout,
    /// 7 用户补充凭证
    UserSupplementaryEvidence,
    /// 8 商家补充凭证
    MerchantSupplementaryEvidence,
    /// 10 待商家处理纠纷
    WaitMerchantHandleDispute,
    /// 11 待平台处理
    WaitPlatformHandle,
    /// 12 取消平台介入
    CancelPlatformIntervention,
    /// 13 平台处理中
    PlatformProcessing,
    /// 14 待用户补充凭证
    WaitUserSupplementaryEvidence,
    /// 16 待商家补充凭证
    WaitMerchantSupplementaryEvidence,
    /// 18 待双方补充凭证
    WaitBothPartiesSupplementaryEvidence,
    /// 20 待商家确认
    WaitMerchantConfirm,
    /// 21 商家申诉中
    MerchantAppealing,
    /// 22 调解完成
    MediationComplete,
    /// 23 待平台核实
    WaitPlatformVerify,
    /// 24 重新退款中
    RefundingAgain,
    /// 26 调解关闭
    MediationClosed,
    /// 30 平台判定用户责任
    PlatformJudgmentUserResponsibility,
    /// 31 平台判定商家责任
    PlatformJudgmentMerchantResponsibility,
    /// 32 平台判定双方责任
    PlatformJudgmentBothPartiesResponsibility,
    /// 33 平台判定无责任
    PlatformJudgmentNoResponsibility,
    /// 34 平台判定申诉无效
    PlatformJudgmentAppealInvalid,
    /// 35 平台判定申诉生效
    PlatformJudgmentAppealEffective,
    /// 36 平台判定退款有效
    PlatformJudgmentRefundEffective,
    /// 37 平台判定退款无效
    PlatformJudgmentRefundInvalid,
    /// 50 用户发起退款
    UserInitiateRefund,
    /// 51 商家拒绝退款
    MerchantRefuseRefund,
    /// 52 用户取消申请
    UserCancelApplication,
    /// 56 待买家退货
    WaitBuyerReturnGoods,
    /// 57 退货退款关闭
    RefundClosed,
    /// 58 待商家收货
    WaitMerchantReceiveGoods,
    /// 59 商家逾期未退款
    MerchantOverdueRefund,
    /// 60 退款完成
    RefundComplete,
    /// 61 退货退款完成
    RefundGoodsComplete,
    /// 62 平台退款中
    PlatformRefunding,
    /// 63 平台退款失败
    PlatformRefundFailed,
    /// 64 待用户确认
    WaitUserConfirm,
}

impl ComplaintItemType {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            ComplaintItemType::ApplyPlatformIntervention => 1,
            ComplaintItemType::UserMessage => 2,
            ComplaintItemType::MerchantMessage => 3,
            ComplaintItemType::SubmitComplaintSuccess => 4,
            ComplaintItemType::ComplaintCancelled => 5,
            ComplaintItemType::MerchantTimeout => 6,
            ComplaintItemType::UserSupplementaryEvidence => 7,
            ComplaintItemType::MerchantSupplementaryEvidence => 8,
            ComplaintItemType::WaitMerchantHandleDispute => 10,
            ComplaintItemType::WaitPlatformHandle => 11,
            ComplaintItemType::CancelPlatformIntervention => 12,
            ComplaintItemType::PlatformProcessing => 13,
            ComplaintItemType::WaitUserSupplementaryEvidence => 14,
            ComplaintItemType::WaitMerchantSupplementaryEvidence => 16,
            ComplaintItemType::WaitBothPartiesSupplementaryEvidence => 18,
            ComplaintItemType::WaitMerchantConfirm => 20,
            ComplaintItemType::MerchantAppealing => 21,
            ComplaintItemType::MediationComplete => 22,
            ComplaintItemType::WaitPlatformVerify => 23,
            ComplaintItemType::RefundingAgain => 24,
            ComplaintItemType::MediationClosed => 26,
            ComplaintItemType::PlatformJudgmentUserResponsibility => 30,
            ComplaintItemType::PlatformJudgmentMerchantResponsibility => 31,
            ComplaintItemType::PlatformJudgmentBothPartiesResponsibility => 32,
            ComplaintItemType::PlatformJudgmentNoResponsibility => 33,
            ComplaintItemType::PlatformJudgmentAppealInvalid => 34,
            ComplaintItemType::PlatformJudgmentAppealEffective => 35,
            ComplaintItemType::PlatformJudgmentRefundEffective => 36,
            ComplaintItemType::PlatformJudgmentRefundInvalid => 37,
            ComplaintItemType::UserInitiateRefund => 50,
            ComplaintItemType::MerchantRefuseRefund => 51,
            ComplaintItemType::UserCancelApplication => 52,
            ComplaintItemType::WaitBuyerReturnGoods => 56,
            ComplaintItemType::RefundClosed => 57,
            ComplaintItemType::WaitMerchantReceiveGoods => 58,
            ComplaintItemType::MerchantOverdueRefund => 59,
            ComplaintItemType::RefundComplete => 60,
            ComplaintItemType::RefundGoodsComplete => 61,
            ComplaintItemType::PlatformRefunding => 62,
            ComplaintItemType::PlatformRefundFailed => 63,
            ComplaintItemType::WaitUserConfirm => 64,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            ComplaintItemType::ApplyPlatformIntervention => "申请平台介入",
            ComplaintItemType::UserMessage => "用户留言",
            ComplaintItemType::MerchantMessage => "商家留言",
            ComplaintItemType::SubmitComplaintSuccess => "提交投诉成功",
            ComplaintItemType::ComplaintCancelled => "投诉已取消",
            ComplaintItemType::MerchantTimeout => "商家已超时",
            ComplaintItemType::UserSupplementaryEvidence => "用户补充凭证",
            ComplaintItemType::MerchantSupplementaryEvidence => "商家补充凭证",
            ComplaintItemType::WaitMerchantHandleDispute => "待商家处理纠纷",
            ComplaintItemType::WaitPlatformHandle => "待平台处理",
            ComplaintItemType::CancelPlatformIntervention => "取消平台介入",
            ComplaintItemType::PlatformProcessing => "平台处理中",
            ComplaintItemType::WaitUserSupplementaryEvidence => "待用户补充凭证",
            ComplaintItemType::WaitMerchantSupplementaryEvidence => "待商家补充凭证",
            ComplaintItemType::WaitBothPartiesSupplementaryEvidence => "待双方补充凭证",
            ComplaintItemType::WaitMerchantConfirm => "待商家确认",
            ComplaintItemType::MerchantAppealing => "商家申诉中",
            ComplaintItemType::MediationComplete => "调解完成",
            ComplaintItemType::WaitPlatformVerify => "待平台核实",
            ComplaintItemType::RefundingAgain => "重新退款中",
            ComplaintItemType::MediationClosed => "调解关闭",
            ComplaintItemType::PlatformJudgmentUserResponsibility => "平台判定用户责任",
            ComplaintItemType::PlatformJudgmentMerchantResponsibility => "平台判定商家责任",
            ComplaintItemType::PlatformJudgmentBothPartiesResponsibility => "平台判定双方责任",
            ComplaintItemType::PlatformJudgmentNoResponsibility => "平台判定无责任",
            ComplaintItemType::PlatformJudgmentAppealInvalid => "平台判定申诉无效",
            ComplaintItemType::PlatformJudgmentAppealEffective => "平台判定申诉生效",
            ComplaintItemType::PlatformJudgmentRefundEffective => "平台判定退款有效",
            ComplaintItemType::PlatformJudgmentRefundInvalid => "平台判定退款无效",
            ComplaintItemType::UserInitiateRefund => "用户发起退款",
            ComplaintItemType::MerchantRefuseRefund => "商家拒绝退款",
            ComplaintItemType::UserCancelApplication => "用户取消申请",
            ComplaintItemType::WaitBuyerReturnGoods => "待买家退货",
            ComplaintItemType::RefundClosed => "退货退款关闭",
            ComplaintItemType::WaitMerchantReceiveGoods => "待商家收货",
            ComplaintItemType::MerchantOverdueRefund => "商家逾期未退款",
            ComplaintItemType::RefundComplete => "退款完成",
            ComplaintItemType::RefundGoodsComplete => "退货退款完成",
            ComplaintItemType::PlatformRefunding => "平台退款中",
            ComplaintItemType::PlatformRefundFailed => "平台退款失败",
            ComplaintItemType::WaitUserConfirm => "待用户确认",
        }
    }
}
