//! 视频号小店消息 bean。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message` 包（50 个类，Wave 2 H2c
//! 迁移）：回调消息数据类（订单/售后/商品/优惠券/资金/会员/分享员/店铺/
//! 团长/团购券），JSON 与 XML 双线格式（`@JsonProperty` + `@JacksonXmlProperty`
//! 的字段名一致，quick-xml serde 与 serde_json 共用同一 `rename`）。
//!
//! Java 的消息类继承 [`crate::message::WxChannelMessage`] 基类；quick-xml 不支持
//! `#[serde(flatten)]` 且 `macro_rules!` 无法展开为结构体字段，故基类 7 个字段
//! 在每个消息类中显式扁平展开（线格式与 Java 完全一致：子类字段与基类字段
//! 处于同一 JSON/XML 对象层级）。

pub mod after;
pub mod coupon;
pub mod fund;
pub mod order;
pub mod product;
pub mod serde_helpers;
pub mod session_message;
pub mod sharer;
pub mod store;
pub mod supplier;
pub mod vip;
pub mod voucher;

pub use session_message::SessionMessage;

pub use after::after_sale_message::AfterSaleMessage;
pub use after::after_sale_status_info::AfterSaleStatusInfo;
pub use after::complaint_info::ComplaintInfo;
pub use after::complaint_message::ComplaintMessage;

pub use coupon::coupon_action_info::CouponActionInfo;
pub use coupon::coupon_action_message::CouponActionMessage;
pub use coupon::coupon_receive_message::CouponReceiveMessage;
pub use coupon::user_coupon_action_info::UserCouponActionInfo;
pub use coupon::user_coupon_expire_message::UserCouponExpireMessage;
pub use coupon::user_coupon_use_message::UserCouponUseMessage;

pub use fund::account_notify_message::AccountNotifyMessage;
pub use fund::bank_notify_info::BankNotifyInfo;
pub use fund::qr_notify_info::QrNotifyInfo;
pub use fund::qr_notify_message::QrNotifyMessage;
pub use fund::withdraw_notify_info::WithdrawNotifyInfo;
pub use fund::withdraw_notify_message::WithdrawNotifyMessage;

pub use order::order_cancel_info::OrderCancelInfo;
pub use order::order_cancel_message::OrderCancelMessage;
pub use order::order_confirm_info::OrderConfirmInfo;
pub use order::order_confirm_message::OrderConfirmMessage;
pub use order::order_delivery_info::OrderDeliveryInfo;
pub use order::order_delivery_message::OrderDeliveryMessage;
pub use order::order_ext_info::OrderExtInfo;
pub use order::order_ext_message::OrderExtMessage;
pub use order::order_id_info::OrderIdInfo;
pub use order::order_id_message::OrderIdMessage;
pub use order::order_pay_info::OrderPayInfo;
pub use order::order_pay_message::OrderPayMessage;
pub use order::order_settle_info::OrderSettleInfo;
pub use order::order_settle_message::OrderSettleMessage;
pub use order::order_status_message::OrderStatusMessage;

pub use product::brand_message::BrandMessage;
pub use product::category_audit_message::CategoryAuditMessage;
pub use product::spu_audit_message::SpuAuditMessage;
pub use product::spu_status_message::SpuStatusMessage;
pub use product::spu_stock_message::SpuStockMessage;

pub use sharer::sharer_change_message::SharerChangeMessage;

pub use store::close_store_message::CloseStoreMessage;
pub use store::nickname_update_message::NicknameUpdateMessage;

pub use supplier::supplier_item_info::SupplierItemInfo;
pub use supplier::supplier_item_message::SupplierItemMessage;

pub use vip::coupon_info::CouponInfo;
pub use vip::exchange_info::ExchangeInfo;
pub use vip::exchange_info_message::ExchangeInfoMessage;
pub use vip::product_info::ProductInfo;
pub use vip::user_info::UserInfo;
pub use vip::user_info_message::UserInfoMessage;

pub use voucher::voucher_info::VoucherInfo;
pub use voucher::voucher_message::VoucherMessage;
