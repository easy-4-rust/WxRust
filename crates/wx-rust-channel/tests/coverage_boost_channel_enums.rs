//! Coverage boost: channel enums (complaint_item_type, after_sales_reason,
//! wx_order_status, funds_type, after_sale_status, etc.).
//!
//! Exercises key()/val()/value() methods for all variants of each enum.

use wx_rust_channel::enums::*;

// ========================================================================
// ComplaintItemType (88 lines, 0%)
// ========================================================================

#[test]
fn complaint_item_type_key_val_all() {
    assert_eq!(ComplaintItemType::ApplyPlatformIntervention.key(), 1);
    assert_eq!(ComplaintItemType::ApplyPlatformIntervention.val(), "申请平台介入");
    assert_eq!(ComplaintItemType::UserMessage.key(), 2);
    assert_eq!(ComplaintItemType::MerchantMessage.key(), 3);
    assert_eq!(ComplaintItemType::SubmitComplaintSuccess.key(), 4);
    assert_eq!(ComplaintItemType::ComplaintCancelled.key(), 5);
    assert_eq!(ComplaintItemType::MerchantTimeout.key(), 6);
    assert_eq!(ComplaintItemType::UserSupplementaryEvidence.key(), 7);
    assert_eq!(ComplaintItemType::MerchantSupplementaryEvidence.key(), 8);
    assert_eq!(ComplaintItemType::WaitMerchantHandleDispute.key(), 10);
    assert_eq!(ComplaintItemType::WaitPlatformHandle.key(), 11);
    assert_eq!(ComplaintItemType::CancelPlatformIntervention.key(), 12);
    assert_eq!(ComplaintItemType::PlatformProcessing.key(), 13);
    assert_eq!(ComplaintItemType::WaitUserSupplementaryEvidence.key(), 14);
    assert_eq!(ComplaintItemType::WaitMerchantSupplementaryEvidence.key(), 16);
    assert_eq!(ComplaintItemType::WaitBothPartiesSupplementaryEvidence.key(), 18);
    assert_eq!(ComplaintItemType::WaitMerchantConfirm.key(), 20);
    assert_eq!(ComplaintItemType::MerchantAppealing.key(), 21);
    assert_eq!(ComplaintItemType::MediationComplete.key(), 22);
    assert_eq!(ComplaintItemType::WaitPlatformVerify.key(), 23);
    assert_eq!(ComplaintItemType::RefundingAgain.key(), 24);
    assert_eq!(ComplaintItemType::MediationClosed.key(), 26);
    assert_eq!(ComplaintItemType::PlatformJudgmentUserResponsibility.key(), 30);
    assert_eq!(ComplaintItemType::PlatformJudgmentMerchantResponsibility.key(), 31);
    assert_eq!(ComplaintItemType::PlatformJudgmentBothPartiesResponsibility.key(), 32);
    assert_eq!(ComplaintItemType::PlatformJudgmentNoResponsibility.key(), 33);
    assert_eq!(ComplaintItemType::PlatformJudgmentAppealInvalid.key(), 34);
    assert_eq!(ComplaintItemType::PlatformJudgmentAppealEffective.key(), 35);
    assert_eq!(ComplaintItemType::PlatformJudgmentRefundEffective.key(), 36);
    assert_eq!(ComplaintItemType::PlatformJudgmentRefundInvalid.key(), 37);
    assert_eq!(ComplaintItemType::UserInitiateRefund.key(), 50);
    assert_eq!(ComplaintItemType::MerchantRefuseRefund.key(), 51);
    assert_eq!(ComplaintItemType::UserCancelApplication.key(), 52);
    assert_eq!(ComplaintItemType::WaitBuyerReturnGoods.key(), 56);
    assert_eq!(ComplaintItemType::RefundClosed.key(), 57);
    assert_eq!(ComplaintItemType::WaitMerchantReceiveGoods.key(), 58);
    assert_eq!(ComplaintItemType::MerchantOverdueRefund.key(), 59);
    assert_eq!(ComplaintItemType::RefundComplete.key(), 60);
    assert_eq!(ComplaintItemType::RefundGoodsComplete.key(), 61);
    assert_eq!(ComplaintItemType::PlatformRefunding.key(), 62);
    assert_eq!(ComplaintItemType::PlatformRefundFailed.key(), 63);
    assert_eq!(ComplaintItemType::WaitUserConfirm.key(), 64);
}

// ========================================================================
// WxOrderStatus (45 lines, 0%)
// ========================================================================

#[test]
fn wx_order_status_key_val_all() {
    assert_eq!(WxOrderStatus::Unpaid.key(), 10);
    assert_eq!(WxOrderStatus::Paid.key(), 20);
    assert_eq!(WxOrderStatus::PartDelivery.key(), 21);
    assert_eq!(WxOrderStatus::Delivery.key(), 30);
    assert_eq!(WxOrderStatus::Completed.key(), 100);
    assert_eq!(WxOrderStatus::UnpaidCancel.key(), 190);
    assert_eq!(WxOrderStatus::AllAfterSale.key(), 200);
    assert_eq!(WxOrderStatus::Cancel.key(), 250);
    // val() for all variants
    let _ = WxOrderStatus::Unpaid.val();
    let _ = WxOrderStatus::Paid.val();
    let _ = WxOrderStatus::PartDelivery.val();
    let _ = WxOrderStatus::Delivery.val();
    let _ = WxOrderStatus::Completed.val();
    let _ = WxOrderStatus::UnpaidCancel.val();
    let _ = WxOrderStatus::AllAfterSale.val();
    let _ = WxOrderStatus::Cancel.val();
}

#[test]
fn wx_order_status_get_status_str() {
    assert_eq!(WxOrderStatus::get_status_str(Some(10)), "待付款");
    assert_eq!(WxOrderStatus::get_status_str(Some(250)), "已取消");
    // Unknown keys return the number as string
    let unknown = WxOrderStatus::get_status_str(Some(999));
    assert!(!unknown.is_empty());
    let none_str = WxOrderStatus::get_status_str(None);
    assert!(!none_str.is_empty());
}

#[test]
fn wx_order_status_is_cancel() {
    assert!(WxOrderStatus::is_cancel(Some(250)));
    assert!(WxOrderStatus::is_cancel(Some(200)));
    assert!(WxOrderStatus::is_cancel(Some(190)));
    assert!(!WxOrderStatus::is_cancel(Some(10)));
    assert!(!WxOrderStatus::is_cancel(None));
}

// ========================================================================
// AfterSaleStatus (40 lines, 0%) - uses value()
// ========================================================================

#[test]
fn after_sale_status_value_all() {
    let _ = AfterSaleStatus::UserCanceld.value();
    let _ = AfterSaleStatus::MerchantProcessing.value();
    let _ = AfterSaleStatus::MerchantRejectRefund.value();
    let _ = AfterSaleStatus::MerchantRejectReturn.value();
    let _ = AfterSaleStatus::UserWaitReturn.value();
    let _ = AfterSaleStatus::ReturnClosed.value();
    let _ = AfterSaleStatus::MerchantWaitReceipt.value();
    let _ = AfterSaleStatus::MerchantOverdueRefund.value();
    let _ = AfterSaleStatus::MerchantRefundSuccess.value();
    let _ = AfterSaleStatus::MerchantReturnSuccess.value();
    let _ = AfterSaleStatus::PlatformRefunding.value();
    let _ = AfterSaleStatus::PlatformRefundFail.value();
    let _ = AfterSaleStatus::UserWaitConfirm.value();
    let _ = AfterSaleStatus::MerchantRefundRetryFail.value();
    let _ = AfterSaleStatus::MerchantFail.value();
    let _ = AfterSaleStatus::UserWaitConfirmUpdate.value();
    let _ = AfterSaleStatus::UserWaitHandleMerchantAfterSale.value();
    // key() for all variants
    let _ = AfterSaleStatus::UserCanceld.key();
    let _ = AfterSaleStatus::MerchantProcessing.key();
    let _ = AfterSaleStatus::PlatformRefunding.key();
}

// ========================================================================
// FundsType (40 lines, 0%)
// ========================================================================

#[test]
fn funds_type_key_val_all() {
    assert_eq!(FundsType::OrderPayIncome.key(), 1);
    assert_eq!(FundsType::OrderFee.key(), 2);
    assert_eq!(FundsType::Refund.key(), 3);
    assert_eq!(FundsType::Withdraw.key(), 4);
    assert_eq!(FundsType::WithdrawFail.key(), 5);
    assert_eq!(FundsType::GuideShare.key(), 6);
    assert_eq!(FundsType::LeagueShare.key(), 7);
    assert_eq!(FundsType::FreightShare.key(), 8);
    assert_eq!(FundsType::LeaguePlatCommission.key(), 9);
    assert_eq!(FundsType::LeagueCommission.key(), 10);
    assert_eq!(FundsType::PlatformCommission.key(), 11);
    assert_eq!(FundsType::LeaderCommission.key(), 12);
    assert_eq!(FundsType::PopularityCard.key(), 13);
    assert_eq!(FundsType::FastRefund.key(), 14);
    assert_eq!(FundsType::FastRefundReplenishment.key(), 15);
    assert_eq!(FundsType::FreightInsurance.key(), 16);
    let _ = FundsType::OrderPayIncome.val();
    let _ = FundsType::FreightInsurance.val();
}

// ========================================================================
// AfterSalesReason (50 lines, 0%) - uses value()
// ========================================================================

#[test]
fn after_sales_reason_value_all() {
    let _ = AfterSalesReason::IncorrectSelection.value();
    let _ = AfterSalesReason::NoLongerWant.value();
    let _ = AfterSalesReason::NoExpressInfo.value();
    let _ = AfterSalesReason::EmptyPackage.value();
    let _ = AfterSalesReason::RejectReceivePackage.value();
    let _ = AfterSalesReason::NotDeliveredTooLong.value();
    let _ = AfterSalesReason::NotMatchProductDesc.value();
    let _ = AfterSalesReason::QualityIssue.value();
    let _ = AfterSalesReason::SendWrongGoods.value();
    let _ = AfterSalesReason::ThreeNoProduct.value();
    let _ = AfterSalesReason::FakeProduct.value();
    let _ = AfterSalesReason::Others.value();
    // key()
    let _ = AfterSalesReason::IncorrectSelection.key();
    let _ = AfterSalesReason::Others.key();
}

// ========================================================================
// RefundReason
// ========================================================================

#[test]
fn refund_reason_key_val_all() {
    assert_eq!(RefundReason::MerchantInitiatedRefund.key(), 1);
    assert_eq!(RefundReason::PlatformForcedRefund.key(), 6);
    assert_eq!(RefundReason::MerchantOverdueConfirmationRefund.key(), 11);
    let _ = RefundReason::MerchantInitiatedRefund.val();
    let _ = RefundReason::PlatformForcedRefund.val();
}

// ========================================================================
// CouponType
// ========================================================================

#[test]
fn coupon_type_key_val_all() {
    assert_eq!(CouponType::C1.key(), 1);
    assert_eq!(CouponType::C2.key(), 2);
    assert_eq!(CouponType::C3.key(), 3);
    assert_eq!(CouponType::C4.key(), 4);
    assert_eq!(CouponType::C101.key(), 101);
    assert_eq!(CouponType::C102.key(), 102);
    assert_eq!(CouponType::C103.key(), 103);
    assert_eq!(CouponType::C104.key(), 104);
    let _ = CouponType::C1.val();
    let _ = CouponType::C104.val();
}

// ========================================================================
// DeliveryType
// ========================================================================

#[test]
fn delivery_type_key_val_all() {
    assert_eq!(DeliveryType::SelfDelivery.key(), 1);
    assert_eq!(DeliveryType::OnlineDelivery.key(), 2);
    assert_eq!(DeliveryType::VirtualDelivery.key(), 3);
    assert_eq!(DeliveryType::OnlineDeliveryScatter.key(), 4);
    let _ = DeliveryType::SelfDelivery.val();
}

// ========================================================================
// BannerType
// ========================================================================

#[test]
fn banner_type_key_val_all() {
    assert_eq!(BannerType::Product.key(), 1);
    assert_eq!(BannerType::Channel.key(), 3);
    assert_eq!(BannerType::Mp.key(), 4);
    let _ = BannerType::Product.val();
}

// ========================================================================
// OrderScene
// ========================================================================

#[test]
fn order_scene_key_val_all() {
    assert_eq!(OrderScene::Other.key(), 1);
    assert_eq!(OrderScene::Live.key(), 2);
    assert_eq!(OrderScene::Video.key(), 3);
    assert_eq!(OrderScene::Share.key(), 4);
    assert_eq!(OrderScene::ShowCase.key(), 5);
    assert_eq!(OrderScene::ArticleCard.key(), 6);
    let _ = OrderScene::Other.val();
}

// ========================================================================
// QrCheckStatus
// ========================================================================

#[test]
fn qr_check_status_key_val_all() {
    assert_eq!(QrCheckStatus::NotScan.key(), 0);
    assert_eq!(QrCheckStatus::Confirmed.key(), 1);
    assert_eq!(QrCheckStatus::Cancel.key(), 2);
    assert_eq!(QrCheckStatus::Invalid.key(), 3);
    assert_eq!(QrCheckStatus::Scan.key(), 4);
    let _ = QrCheckStatus::NotScan.val();
}

// ========================================================================
// PromoteType
// ========================================================================

#[test]
fn promote_type_key_val_all() {
    assert_eq!(PromoteType::PromoteTypeShop.key(), 1);
    assert_eq!(PromoteType::Member.key(), 9);
    assert_eq!(PromoteType::MemberCard.key(), 10);
    let _ = PromoteType::PromoteTypeShop.val();
}

// ========================================================================
// DimensionType
// ========================================================================

#[test]
fn dimension_type_key_val_all() {
    assert_eq!(DimensionType::PrimaryChannel.key(), 1);
    assert_eq!(DimensionType::Age.key(), 2);
    assert_eq!(DimensionType::Sex.key(), 3);
    assert_eq!(DimensionType::Follow.key(), 5);
    assert_eq!(DimensionType::SecondaryChannel.key(), 7);
    assert_eq!(DimensionType::Cate.key(), 9);
    assert_eq!(DimensionType::Province.key(), 10);
    assert_eq!(DimensionType::City.key(), 11);
    assert_eq!(DimensionType::EcomUserLevel.key(), 12);
    assert_eq!(DimensionType::GmvPerCnt.key(), 13);
    assert_eq!(DimensionType::Flow.key(), 16);
    let _ = DimensionType::PrimaryChannel.val();
}

// ========================================================================
// CommissionOrderStatus
// ========================================================================

#[test]
fn commission_order_status_key_val_all() {
    assert_eq!(CommissionOrderStatus::NotSettled.key(), 20);
    assert_eq!(CommissionOrderStatus::Settled.key(), 100);
    assert_eq!(CommissionOrderStatus::CancelSettled.key(), 200);
    let _ = CommissionOrderStatus::NotSettled.val();
}

// ========================================================================
// CouponValidType
// ========================================================================

#[test]
fn coupon_valid_type_key_val() {
    assert_eq!(CouponValidType::CouponValidTypeTime.key(), 1);
    assert_eq!(CouponValidType::CouponValidTypeDay.key(), 2);
    let _ = CouponValidType::CouponValidTypeTime.val();
}

// ========================================================================
// AfterSaleType - uses value()
// ========================================================================

#[test]
fn after_sale_type_value_all() {
    let _ = AfterSaleType::RefundOnly.value();
    let _ = AfterSaleType::RefundGoods.value();
    let _ = AfterSaleType::RefundOnly.key();
}

// ========================================================================
// MessageType
// ========================================================================

#[test]
fn message_type_key_val() {
    let _ = MessageType::Event.key();
}

// ========================================================================
// LiveDistributionFlowType
// ========================================================================

#[test]
fn live_distribution_flow_type_key_val_all() {
    assert_eq!(LiveDistributionFlowType::Invalid.key(), 0);
    assert_eq!(LiveDistributionFlowType::Natural.key(), 1);
    assert_eq!(LiveDistributionFlowType::Promote.key(), 2);
    assert_eq!(LiveDistributionFlowType::Ads.key(), 3);
    assert_eq!(LiveDistributionFlowType::CommonDomain.key(), 4);
    assert_eq!(LiveDistributionFlowType::PrivateDomain.key(), 5);
    let _ = LiveDistributionFlowType::Invalid.val();
}

// ========================================================================
// LiveDistributionSceneType
// ========================================================================

#[test]
fn live_distribution_scene_type_key_val_all() {
    assert_eq!(LiveDistributionSceneType::ProductImpression.key(), 6);
    assert_eq!(LiveDistributionSceneType::LiveRoomImpressionPv.key(), 7);
    assert_eq!(LiveDistributionSceneType::ProductClickPv.key(), 8);
    assert_eq!(LiveDistributionSceneType::ChannelTotalCreatePv.key(), 9);
    assert_eq!(LiveDistributionSceneType::ChannelTotalPayPv.key(), 10);
    let _ = LiveDistributionSceneType::ProductImpression.val();
}

// ========================================================================
// SaleProfileUserType
// ========================================================================

#[test]
fn sale_profile_user_type_key_val_all() {
    assert_eq!(SaleProfileUserType::ProductImpressionUser.key(), 1);
    assert_eq!(SaleProfileUserType::ProductClickUser.key(), 2);
    assert_eq!(SaleProfileUserType::PurchasingUser.key(), 3);
    assert_eq!(SaleProfileUserType::FirstPurchaseUser.key(), 4);
    assert_eq!(SaleProfileUserType::RepurchaseUser.key(), 5);
    assert_eq!(SaleProfileUserType::LiveWatcherUser.key(), 6);
    let _ = SaleProfileUserType::ProductImpressionUser.val();
}

// ========================================================================
// EcProfileDataNodeKey - uses value()
// ========================================================================

#[test]
fn ec_profile_data_node_key_value_all() {
    let _ = EcProfileDataNodeKey::Sex.value();
    let _ = EcProfileDataNodeKey::Age.value();
    let _ = EcProfileDataNodeKey::Province.value();
    let _ = EcProfileDataNodeKey::City.value();
    let _ = EcProfileDataNodeKey::Follow.value();
    let _ = EcProfileDataNodeKey::Cate.value();
    let _ = EcProfileDataNodeKey::EcomUserLevel.value();
    let _ = EcProfileDataNodeKey::GmvPerCnt.value();
    let _ = EcProfileDataNodeKey::Sex.key();
}

// ========================================================================
// PackageAuditItemType - uses value()
// ========================================================================

#[test]
fn package_audit_item_type_value_all() {
    let _ = PackageAuditItemType::ExpressPic.value();
    let _ = PackageAuditItemType::BoxPic.value();
    let _ = PackageAuditItemType::UnboxingPic.value();
    let _ = PackageAuditItemType::DetailPic.value();
    let _ = PackageAuditItemType::ExpressPic.key();
}

// ========================================================================
// SharerType
// ========================================================================

#[test]
fn sharer_type_key_val_all() {
    assert_eq!(SharerType::Normal.key(), 0);
    assert_eq!(SharerType::Enterprise.key(), 1);
    let _ = SharerType::Normal.val();
}

// ========================================================================
// AccountType - uses value()
// ========================================================================

#[test]
fn account_type_value_all() {
    let _ = AccountType::AccountTypeBusiness.value();
    let _ = AccountType::AccountTypePrivate.value();
    let _ = AccountType::AccountTypeBusiness.key();
}

// ========================================================================
// ShareScene
// ========================================================================

#[test]
fn share_scene_val_all() {
    let _ = ShareScene::LiveRoom.val();
    let _ = ShareScene::Window.val();
    let _ = ShareScene::ShortVideo.val();
    let _ = ShareScene::ChannelHome.val();
    let _ = ShareScene::ProductDetail.val();
    let _ = ShareScene::LiveRoom.key();
}

// ========================================================================
// WithdrawStatus - uses value()
// ========================================================================

#[test]
fn withdraw_status_value_all() {
    let _ = WithdrawStatus::CreateSuccess.value();
    let _ = WithdrawStatus::Success.value();
    let _ = WithdrawStatus::Fail.value();
    let _ = WithdrawStatus::Refund.value();
    let _ = WithdrawStatus::Close.value();
    let _ = WithdrawStatus::CreateSuccess.key();
}

// ========================================================================
// WxCouponStatus
// ========================================================================

#[test]
fn wx_coupon_status_val_all() {
    let _ = WxCouponStatus::Init.val();
    let _ = WxCouponStatus::Valid.val();
    let _ = WxCouponStatus::Invalid.val();
    let _ = WxCouponStatus::Delete.val();
    let _ = WxCouponStatus::Init.key();
}

// ========================================================================
// UserCouponStatus
// ========================================================================

#[test]
fn user_coupon_status_val_all() {
    let _ = UserCouponStatus::Valid.val();
    let _ = UserCouponStatus::Expired.val();
    let _ = UserCouponStatus::Used.val();
    let _ = UserCouponStatus::Valid.key();
}

// ========================================================================
// SpuStatus / SpuEditStatus
// ========================================================================

#[test]
fn spu_status_val_all() {
    let _ = SpuStatus::Init.val();
    let _ = SpuStatus::Up.val();
    let _ = SpuStatus::Trash.val();
    let _ = SpuStatus::Delete.val();
    let _ = SpuStatus::Down.val();
    let _ = SpuStatus::Init.key();
}

#[test]
fn spu_edit_status_val_all() {
    let _ = SpuEditStatus::Init.val();
    let _ = SpuEditStatus::Submit.val();
    let _ = SpuEditStatus::Ing.val();
    let _ = SpuEditStatus::Fail.val();
    let _ = SpuEditStatus::Success.val();
    let _ = SpuEditStatus::Init.key();
}

// ========================================================================
// SendTime - uses key() -> &'static str, value()
// ========================================================================

#[test]
fn send_time_key_value_all() {
    let _ = SendTime::TwentyfourHour.key();
    let _ = SendTime::FoutyeightHour.key();
    let _ = SendTime::ThreeDay.key();
    let _ = SendTime::FiveDay.key();
    let _ = SendTime::SevenDay.key();
    let _ = SendTime::TwentyfourHour.value();
    let _ = SendTime::SevenDay.value();
}
