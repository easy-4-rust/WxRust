//! Coverage boost: `wx_channel_message_service.rs` (361 lines, 0%).
//!
//! Exercises all default method implementations on `WxChannelMessageService`
//! via `WxChannelMessageServiceImpl`.

use wx_rust_common::session::{StandardSessionManager, WxSessionManager};

use wx_rust_channel::api::WxChannelMessageService;
use wx_rust_channel::api::r#impl::WxChannelMessageServiceImpl;
use wx_rust_channel::bean::message::after::{AfterSaleMessage, ComplaintMessage};
use wx_rust_channel::bean::message::coupon::{
    CouponActionMessage, CouponReceiveMessage, UserCouponExpireMessage,
};
use wx_rust_channel::bean::message::fund::{
    AccountNotifyMessage, QrNotifyMessage, WithdrawNotifyMessage,
};
use wx_rust_channel::bean::message::order::{
    OrderCancelMessage, OrderConfirmMessage, OrderDeliveryMessage, OrderExtMessage, OrderIdMessage,
    OrderPayMessage, OrderSettleMessage, OrderStatusMessage,
};
use wx_rust_channel::bean::message::product::{
    BrandMessage, CategoryAuditMessage, SpuAuditMessage, SpuStockMessage,
};
use wx_rust_channel::bean::message::store::{CloseStoreMessage, NicknameUpdateMessage};
use wx_rust_channel::bean::message::supplier::SupplierItemMessage;
use wx_rust_channel::bean::message::vip::{ExchangeInfoMessage, UserInfoMessage};
use wx_rust_channel::bean::message::voucher::VoucherMessage;
use wx_rust_channel::message::{RouteContext, WxChannelMessage};

fn svc() -> WxChannelMessageServiceImpl {
    WxChannelMessageServiceImpl::new()
}

fn ctx() -> RouteContext {
    RouteContext::default()
}

fn sm() -> &'static dyn WxSessionManager {
    // Leak a session manager for static lifetime (test only)
    Box::leak(Box::new(StandardSessionManager::new()))
}

#[test]
fn order_new_default() {
    let s = svc();
    s.order_new(&OrderIdMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_cancel_default() {
    let s = svc();
    s.order_cancel(&OrderCancelMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_pay_default() {
    let s = svc();
    s.order_pay(&OrderPayMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_wait_shipping_default() {
    let s = svc();
    s.order_wait_shipping(&OrderIdMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_delivery_default() {
    let s = svc();
    s.order_delivery(&OrderDeliveryMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_confirm_default() {
    let s = svc();
    s.order_confirm(&OrderConfirmMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_settle_default() {
    let s = svc();
    s.order_settle(&OrderSettleMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_ext_info_update_default() {
    let s = svc();
    s.order_ext_info_update(&OrderExtMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn order_status_update_default() {
    let s = svc();
    s.order_status_update(&OrderStatusMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn spu_audit_default() {
    let s = svc();
    s.spu_audit(&SpuAuditMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn spu_status_update_default() {
    let s = svc();
    s.spu_status_update(&SpuAuditMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn spu_update_default() {
    let s = svc();
    s.spu_update(&SpuAuditMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn stock_no_enough_default() {
    let s = svc();
    s.stock_no_enough(&SpuStockMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn category_audit_default() {
    let s = svc();
    s.category_audit(&CategoryAuditMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn brand_update_default() {
    let s = svc();
    s.brand_update(&BrandMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn after_sale_status_update_default() {
    let s = svc();
    s.after_sale_status_update(&AfterSaleMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn complaint_notify_default() {
    let s = svc();
    s.complaint_notify(&ComplaintMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_receive_default() {
    let s = svc();
    s.coupon_receive(&CouponReceiveMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_create_default() {
    let s = svc();
    s.coupon_create(&CouponActionMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_delete_default() {
    let s = svc();
    s.coupon_delete(&CouponActionMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_expire_default() {
    let s = svc();
    s.coupon_expire(&CouponActionMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_update_default() {
    let s = svc();
    s.coupon_update(&CouponActionMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn coupon_invalid_default() {
    let s = svc();
    s.coupon_invalid(&CouponActionMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn user_coupon_expire_default() {
    let s = svc();
    s.user_coupon_expire(
        &UserCouponExpireMessage::default(),
        "",
        "",
        &mut ctx(),
        sm(),
    );
}

#[test]
fn user_coupon_use_default() {
    let s = svc();
    s.user_coupon_use(
        &UserCouponExpireMessage::default(),
        "",
        "",
        &mut ctx(),
        sm(),
    );
}

#[test]
fn user_coupon_unuse_default() {
    let s = svc();
    s.user_coupon_unuse(
        &UserCouponExpireMessage::default(),
        "",
        "",
        &mut ctx(),
        sm(),
    );
}

#[test]
fn voucher_send_succ_default() {
    let s = svc();
    s.voucher_send_succ(&VoucherMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn account_notify_default() {
    let s = svc();
    s.account_notify(&AccountNotifyMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn withdraw_notify_default() {
    let s = svc();
    s.withdraw_notify(&WithdrawNotifyMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn qr_notify_default() {
    let s = svc();
    s.qr_notify(&QrNotifyMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn supplier_item_update_default() {
    let s = svc();
    s.supplier_item_update(&SupplierItemMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn vip_join_default() {
    let s = svc();
    s.vip_join(&UserInfoMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn vip_close_default() {
    let s = svc();
    s.vip_close(&UserInfoMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn vip_grade_update_default() {
    let s = svc();
    s.vip_grade_update(&UserInfoMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn vip_score_update_default() {
    let s = svc();
    s.vip_score_update(&UserInfoMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn vip_score_exchange_default() {
    let s = svc();
    s.vip_score_exchange(&ExchangeInfoMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn close_store_default() {
    let s = svc();
    s.close_store(&CloseStoreMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn update_nickname_default() {
    let s = svc();
    s.update_nickname(&NicknameUpdateMessage::default(), "", "", &mut ctx(), sm());
}

#[test]
fn default_message_handler_returns_none() {
    let s = svc();
    let result = s.default_message_handler(&WxChannelMessage::default(), "", "", &mut ctx(), sm());
    assert!(result.is_none());
}

#[test]
fn sharer_change_default() {
    let s = svc();
    s.sharer_change(&WxChannelMessage::default(), "", "", &mut ctx(), sm());
}
