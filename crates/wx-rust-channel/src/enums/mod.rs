//! 视频号小店枚举与接口地址常量。
//!
//! 对应 Java `me.chanjar.weixin.channel.enums` 包（业务枚举）与
//! `me.chanjar.weixin.channel.constant.WxChannelApiUrlConstants`（接口地址，
//! 按子域拆分到 `url_*` 模块）。`WxChannelErrorMsgEnum` 已在
//! `wx-rust-common::error::wx_channel_error_msg_enum`（`find_msg_by_code`）迁移，
//! 本模块不重复定义。

pub mod account_type;
pub mod after_sale_status;
pub mod after_sale_type;
pub mod after_sales_reason;
pub mod banner_type;
pub mod commission_order_status;
pub mod complaint_item_type;
pub mod complaint_status;
pub mod coupon_type;
pub mod coupon_valid_type;
pub mod delivery_type;
pub mod dimension_type;
pub mod ec_profile_data_node_key;
pub mod funds_type;
pub mod live_distribution_flow_type;
pub mod live_distribution_scene_type;
pub mod message_type;
pub mod order_scene;
pub mod package_audit_item_type;
pub mod promote_type;
pub mod qr_check_status;
pub mod refund_reason;
pub mod sale_profile_user_type;
pub mod send_time;
pub mod share_scene;
pub mod sharer_type;
pub mod spu_edit_status;
pub mod spu_status;
pub mod user_coupon_status;
pub mod withdraw_status;
pub mod wx_coupon_status;
pub mod wx_order_status;

// 接口地址常量（对应 Java `WxChannelApiUrlConstants`，按子域拆分）。
pub mod url_address;
pub mod url_after_sale;
pub mod url_assistant;
pub mod url_basics;
pub mod url_brand;
pub mod url_category;
pub mod url_compass_finder;
pub mod url_compass_shop;
pub mod url_complaint;
pub mod url_cooperation;
pub mod url_core;
pub mod url_coupon;
pub mod url_delivery;
pub mod url_finder_live;
pub mod url_freight;
pub mod url_funds;
pub mod url_home_page;
pub mod url_lead_component;
pub mod url_league;
pub mod url_live_dashboard;
pub mod url_order;
pub mod url_product;
pub mod url_sharer;
pub mod url_vip;
pub mod url_warehouse;

pub use account_type::AccountType;
pub use after_sale_status::AfterSaleStatus;
pub use after_sale_type::AfterSaleType;
pub use after_sales_reason::AfterSalesReason;
pub use banner_type::BannerType;
pub use commission_order_status::CommissionOrderStatus;
pub use complaint_item_type::ComplaintItemType;
pub use complaint_status::ComplaintStatus;
pub use coupon_type::CouponType;
pub use coupon_valid_type::CouponValidType;
pub use delivery_type::DeliveryType;
pub use dimension_type::DimensionType;
pub use ec_profile_data_node_key::EcProfileDataNodeKey;
pub use funds_type::FundsType;
pub use live_distribution_flow_type::LiveDistributionFlowType;
pub use live_distribution_scene_type::LiveDistributionSceneType;
pub use message_type::MessageType;
pub use order_scene::OrderScene;
pub use package_audit_item_type::PackageAuditItemType;
pub use promote_type::PromoteType;
pub use qr_check_status::QrCheckStatus;
pub use refund_reason::RefundReason;
pub use sale_profile_user_type::SaleProfileUserType;
pub use send_time::SendTime;
pub use share_scene::ShareScene;
pub use sharer_type::SharerType;
pub use spu_edit_status::SpuEditStatus;
pub use spu_status::SpuStatus;
pub use user_coupon_status::UserCouponStatus;
pub use withdraw_status::WithdrawStatus;
pub use wx_coupon_status::WxCouponStatus;
pub use wx_order_status::WxOrderStatus;
