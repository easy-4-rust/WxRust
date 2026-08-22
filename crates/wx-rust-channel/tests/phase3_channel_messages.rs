//! Phase 3 P2 扩展: channel 消息（Message）子系统扩展测试。
//!
//! 镜像 Java:
//! - `WxChannelMessageRouterTest`（卡券/资金/商品/分享员/小店/团购券消息路由）
//! - `CouponActionMessageTest`（卡券操作消息解析）
//! - `WithdrawNotifyMessageTest`（提现通知消息解析）
//! - `AccountNotifyMessageTest`（账户变更消息解析）
//! - `SharerChangeMessageTest`（分享员变更消息解析）
//! - `CloseStoreMessageTest`（小店注销消息解析）
//! - `VoucherMessageTest`（团购券消息解析）
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java @Test 的序列化/反序列化断言
//! - RUST_OBLIGATION: serde rename（大驼峰 ToUserName 等）、i64/i32 兼容
//! - VALUE_ADD: 空值/边界/MsgID 别名路径

use wx_rust_channel::bean::message::after::AfterSaleMessage;
use wx_rust_channel::bean::message::coupon::{
    CouponActionMessage, CouponReceiveMessage, UserCouponExpireMessage, UserCouponUseMessage,
};
use wx_rust_channel::bean::message::fund::{
    AccountNotifyMessage, QrNotifyMessage, WithdrawNotifyMessage,
};
use wx_rust_channel::bean::message::product::{
    BrandMessage, CategoryAuditMessage, SpuStatusMessage, SpuStockMessage,
};
use wx_rust_channel::bean::message::session_message::SessionMessage;
use wx_rust_channel::bean::message::sharer::SharerChangeMessage;
use wx_rust_channel::bean::message::store::{CloseStoreMessage, NicknameUpdateMessage};
use wx_rust_channel::bean::message::voucher::VoucherMessage;

// ═══════════════════════════════════════════════════════════════
// 1. 卡券操作消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (coupon action golden)）
// ═══════════════════════════════════════════════════════════════

/// 卡券操作消息 JSON golden（对应 Java `CouponActionMessage`：
/// 基类字段 + `coupon_info` 嵌套 `CouponActionInfo`）。
/// 对应 Java: WxChannelMessageRouterTest (coupon action)
#[test]
fn test_coupon_action_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_coupon_action",
        "MsgId":1001,
        "coupon_info":{
            "coupon_id":"COUPON-001",
            "create_time":"2024-01-01T00:00:00+08:00",
            "change_time":"2024-01-02T00:00:00+08:00"
        }
    }"#;
    let msg: CouponActionMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.to_user.as_deref(), Some("gh_test"));
    assert_eq!(msg.from_user.as_deref(), Some("ox123"));
    assert_eq!(msg.create_time, Some(1662480000));
    assert_eq!(msg.event.as_deref(), Some("channels_ec_coupon_action"));
    assert_eq!(msg.msg_id, Some(1001));
    let info = msg.coupon_info.as_ref().unwrap();
    assert_eq!(info.coupon_id.as_deref(), Some("COUPON-001"));
}

/// MsgID 大写兼容别名（对应 Java `msgIdFill` setter）。
#[test]
fn test_coupon_action_message_msg_id_alias() {
    let json = r#"{"MsgID":2001,"Event":"channels_ec_coupon_action"}"#;
    let msg: CouponActionMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.msg_id, Some(2001));
}

/// 卡券操作消息 CreateTime 字符串兼容。
#[test]
fn test_coupon_action_message_create_time_string() {
    let json = r#"{"CreateTime":"1662480000","Event":"channels_ec_coupon_action"}"#;
    let msg: CouponActionMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.create_time, Some(1662480000));
}

// ═══════════════════════════════════════════════════════════════
// 2. 卡券领取消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (coupon receive golden)）
// ═══════════════════════════════════════════════════════════════

/// 卡券领取消息 JSON golden。
/// 对应 Java: WxChannelMessageRouterTest (coupon receive)
#[test]
fn test_coupon_receive_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_coupon_receive",
        "MsgId":1002,
        "coupon_info":{"coupon_id":"COUPON-002","create_time":"2024-01-01"}
    }"#;
    let msg: CouponReceiveMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_coupon_receive"));
}

/// 用户卡券过期消息。
/// 对应 Java: WxChannelMessageRouterTest (coupon expire)
#[test]
fn test_user_coupon_expire_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_coupon_expire",
        "coupon_info":{"coupon_id":"COUPON-003","expire_time":"2024-12-31"}
    }"#;
    let msg: UserCouponExpireMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_coupon_expire"));
}

/// 用户卡券核销消息。
/// 对应 Java: WxChannelMessageRouterTest (coupon use)
#[test]
fn test_user_coupon_use_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_coupon_use",
        "coupon_info":{"coupon_id":"COUPON-004"}
    }"#;
    let msg: UserCouponUseMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_coupon_use"));
}

// ═══════════════════════════════════════════════════════════════
// 3. 提现通知消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (withdraw golden)）
// ═══════════════════════════════════════════════════════════════

/// 提现通知消息 JSON golden（对应 Java `WithdrawNotifyMessage`：
/// 基类字段 + `withdraw_info` 嵌套 `WithdrawNotifyInfo`）。
/// 对应 Java: WxChannelMessageRouterTest (withdraw notify)
#[test]
fn test_withdraw_notify_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_fund_withdraw",
        "MsgId":1003,
        "withdraw_info":{"event":3,"withdraw_id":"WD-001"}
    }"#;
    let msg: WithdrawNotifyMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_fund_withdraw"));
    let info = msg.withdraw_info.as_ref().unwrap();
    assert_eq!(info.event, Some(3));
    assert_eq!(info.withdraw_id.as_deref(), Some("WD-001"));
}

/// 提现通知消息 event 字符串兼容（对应 Java `opt_string_or_i32`）。
#[test]
fn test_withdraw_notify_message_event_string() {
    let json = r#"{
        "Event":"channels_ec_fund_withdraw",
        "withdraw_info":{"event":"3","withdraw_id":"WD-002"}
    }"#;
    let msg: WithdrawNotifyMessage = serde_json::from_str(json).unwrap();
    let info = msg.withdraw_info.as_ref().unwrap();
    assert_eq!(info.event, Some(3));
}

// ═══════════════════════════════════════════════════════════════
// 4. 账户变更通知消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (account golden)）
// ═══════════════════════════════════════════════════════════════

/// 账户变更通知消息 JSON golden（对应 Java `AccountNotifyMessage`：
/// 基类字段 + `account_info` 嵌套 `BankNotifyInfo`）。
/// 对应 Java: WxChannelMessageRouterTest (account notify)
#[test]
fn test_account_notify_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_fund_change",
        "MsgId":1004,
        "account_info":{"change_type":1,"change_amount":1000,"balance":50000}
    }"#;
    let msg: AccountNotifyMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_fund_change"));
    assert!(msg.account_info.is_some());
}

/// 二维码通知消息（对应 Java `QrNotifyMessage`）。
#[test]
fn test_qr_notify_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_fund_qr",
        "qr_info":{"qr_code":"https://example.com/qr"}
    }"#;
    let msg: QrNotifyMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_fund_qr"));
}

// ═══════════════════════════════════════════════════════════════
// 5. 商品消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (product golden)）
// ═══════════════════════════════════════════════════════════════

/// 品牌审核消息 JSON golden（对应 Java `BrandMessage`）。
/// 对应 Java: WxChannelMessageRouterTest (brand golden)
#[test]
fn test_brand_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_brand_audit",
        "brand_info":{"brand_id":"B001","status":1}
    }"#;
    let msg: BrandMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_brand_audit"));
}

/// 类目审核消息（对应 Java `CategoryAuditMessage`）。
#[test]
fn test_category_audit_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_category_audit",
        "audit_info":{"category_id":"CAT-001","status":2}
    }"#;
    let msg: CategoryAuditMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_category_audit"));
}

/// SPU状态变更消息（对应 Java `SpuStatusMessage`）。
#[test]
fn test_spu_status_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_product_spu_update",
        "spu_info":{"product_id":"SPU-001","status":1}
    }"#;
    let msg: SpuStatusMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_product_spu_update"));
}

/// SPU库存变更消息（对应 Java `SpuStockMessage`）。
#[test]
fn test_spu_stock_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_product_spu_stock",
        "stock_info":{"product_id":"SPU-001","stock_num":100}
    }"#;
    let msg: SpuStockMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_product_spu_stock"));
}

// ═══════════════════════════════════════════════════════════════
// 6. 分享员变更消息（SOURCE_PARITY:
//    Java SharerChangeMessageTest）
// ═══════════════════════════════════════════════════════════════

/// 分享员变更消息 JSON golden（对应 Java `SharerChangeMessage`：
/// 基类字段 + `openid`/`sharer_type`/`bind_status`）。
/// 对应 Java: SharerChangeMessageTest
#[test]
fn test_sharer_change_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_sharer_change",
        "MsgId":1005,
        "openid":"ox_sharer",
        "sharer_type":0,
        "bind_status":1
    }"#;
    let msg: SharerChangeMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_sharer_change"));
    assert_eq!(msg.openid.as_deref(), Some("ox_sharer"));
    assert_eq!(msg.sharer_type, Some(0));
    assert_eq!(msg.bind_status, Some(1));
}

// ═══════════════════════════════════════════════════════════════
// 7. 小店注销消息（SOURCE_PARITY:
//    Java CloseStoreMessageTest）
// ═══════════════════════════════════════════════════════════════

/// 小店注销消息 JSON golden（对应 Java `CloseStoreMessage`：
/// 基类字段 + `appid`/`close_timestamp`）。
/// 对应 Java: CloseStoreMessageTest
#[test]
fn test_close_store_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_store_close",
        "MsgId":1006,
        "appid":"wx1234",
        "close_timestamp":1662480000
    }"#;
    let msg: CloseStoreMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_store_close"));
    assert_eq!(msg.appid.as_deref(), Some("wx1234"));
    assert_eq!(msg.close_timestamp, Some(1662480000));
}

/// 小店注销消息 close_timestamp 字符串兼容。
#[test]
fn test_close_store_message_timestamp_string() {
    let json = r#"{
        "Event":"channels_ec_store_close",
        "appid":"wx1234",
        "close_timestamp":"1662480000"
    }"#;
    let msg: CloseStoreMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.close_timestamp, Some(1662480000));
}

/// 小店昵称修改消息（对应 Java `NicknameUpdateMessage`：
/// 基类字段 + `appid`/`old_nickname`/`new_nickname`）。
/// 对应 Java: NicknameUpdateMessageTest
#[test]
fn test_nickname_update_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_nickname_update",
        "appid":"wx1234",
        "old_nickname":"旧昵称",
        "new_nickname":"新昵称"
    }"#;
    let msg: NicknameUpdateMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_nickname_update"));
    assert_eq!(msg.old_nickname.as_deref(), Some("旧昵称"));
    assert_eq!(msg.new_nickname.as_deref(), Some("新昵称"));
}

// ═══════════════════════════════════════════════════════════════
// 8. 团购券消息（SOURCE_PARITY:
//    Java VoucherMessageTest）
// ═══════════════════════════════════════════════════════════════

/// 团购券消息 JSON golden（对应 Java `VoucherMessage`：
/// 基类字段 + `voucher_list` 数组，每项含 `VoucherInfo`：`code`/`status`/
/// `voucher_type` 等）。
/// 对应 Java: VoucherMessageTest
#[test]
fn test_voucher_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_voucher_verify",
        "voucher_list":[{"code":"V-001","status":1,"voucher_type":2}]
    }"#;
    let msg: VoucherMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.event.as_deref(), Some("channels_ec_voucher_verify"));
    assert!(msg.voucher_info.is_some());
    let vouchers = msg.voucher_info.as_ref().unwrap();
    assert_eq!(vouchers.len(), 1);
    assert_eq!(vouchers[0].code.as_deref(), Some("V-001"));
    assert_eq!(vouchers[0].status, Some(1));
}

/// 团购券消息空列表。
#[test]
fn test_voucher_message_empty_list() {
    let json = r#"{"Event":"channels_ec_voucher_verify","voucher_list":[]}"#;
    let msg: VoucherMessage = serde_json::from_str(json).unwrap();
    let vouchers = msg.voucher_info.as_ref().unwrap();
    assert!(vouchers.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 9. 售后消息（SOURCE_PARITY:
//    Java WxChannelMessageRouterTest (after sale golden)）
// ═══════════════════════════════════════════════════════════════

/// 售后状态更新消息 JSON golden（对应 Java `AfterSaleMessage`：
/// 基类字段 + `finder_shop_aftersale_status_update` 嵌套 `AfterSaleStatusInfo`：
/// `after_sale_order_id`/`status`(String)/`order_id`）。
/// 对应 Java: WxChannelMessageRouterTest (after sale)
#[test]
fn test_after_sale_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"event",
        "Event":"channels_ec_aftersale_status_change",
        "MsgId":1007,
        "finder_shop_aftersale_status_update":{"after_sale_order_id":"AS-001","status":"DOING","order_id":"ORDER-001"}
    }"#;
    let msg: AfterSaleMessage = serde_json::from_str(json).unwrap();
    assert_eq!(
        msg.event.as_deref(),
        Some("channels_ec_aftersale_status_change")
    );
    let info = msg.info.as_ref().unwrap();
    assert_eq!(info.after_sale_order_id.as_deref(), Some("AS-001"));
    assert_eq!(info.status.as_deref(), Some("DOING"));
}

// ═══════════════════════════════════════════════════════════════
// 10. 客服会话消息（SOURCE_PARITY:
//     Java WxChannelMessageRouterTest (session golden)）
// ═══════════════════════════════════════════════════════════════

/// 客服会话消息 JSON golden（对应 Java `SessionMessage`：
/// 基类字段 + `SessionFrom`）。
/// 对应 Java: WxChannelMessageRouterTest (session)
#[test]
fn test_session_message_serde() {
    let json = r#"{
        "ToUserName":"gh_test",
        "FromUserName":"ox123",
        "CreateTime":1662480000,
        "MsgType":"text",
        "SessionFrom":"ox123",
        "MsgId":1008
    }"#;
    let msg: SessionMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.from.as_deref(), Some("ox123"));
}

// ═══════════════════════════════════════════════════════════════
// VALUE_ADD: 边界/空值
// ═══════════════════════════════════════════════════════════════

/// 卡券操作消息默认值。
#[test]
fn test_coupon_action_message_default() {
    let msg: CouponActionMessage = serde_json::from_str("{}").unwrap();
    assert!(msg.to_user.is_none());
    assert!(msg.coupon_info.is_none());
    assert_eq!(msg.msg_id, None);
}

/// 提现通知消息默认值。
#[test]
fn test_withdraw_notify_message_default() {
    let msg: WithdrawNotifyMessage = serde_json::from_str("{}").unwrap();
    assert!(msg.withdraw_info.is_none());
}

/// 分享员变更消息默认值。
#[test]
fn test_sharer_change_message_default() {
    let msg: SharerChangeMessage = serde_json::from_str("{}").unwrap();
    assert!(msg.openid.is_none());
    assert_eq!(msg.sharer_type, None);
    assert_eq!(msg.bind_status, None);
}

/// 小店注销消息默认值。
#[test]
fn test_close_store_message_default() {
    let msg: CloseStoreMessage = serde_json::from_str("{}").unwrap();
    assert!(msg.appid.is_none());
    assert_eq!(msg.close_timestamp, None);
}

/// 卡券操作消息 Encode 字段（对应 Java `WxChannelMessage.encrypt`）。
#[test]
fn test_coupon_action_message_encrypt_field() {
    let json = r#"{"Encrypt":"encrypted_data","Event":"channels_ec_coupon_action"}"#;
    let msg: CouponActionMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.encrypt.as_deref(), Some("encrypted_data"));
}

/// 提现通知 event 字符串 "1" 兼容。
#[test]
fn test_withdraw_notify_event_string_one() {
    let json = r#"{"withdraw_info":{"event":"1","withdraw_id":"WD-003"}}"#;
    let msg: WithdrawNotifyMessage = serde_json::from_str(json).unwrap();
    assert_eq!(msg.withdraw_info.as_ref().unwrap().event, Some(1));
}
