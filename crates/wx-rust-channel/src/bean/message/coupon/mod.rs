//! 视频号小店 优惠券 回调消息。
//!
//! 对应 Java `me.chanjar.weixin.channel.bean.message.coupon` 包。

pub mod coupon_action_info;
pub mod coupon_action_message;
pub mod coupon_receive_message;
pub mod user_coupon_action_info;
pub mod user_coupon_expire_message;
pub mod user_coupon_use_message;

pub use coupon_action_info::CouponActionInfo;
pub use coupon_action_message::CouponActionMessage;
pub use coupon_receive_message::CouponReceiveMessage;
pub use user_coupon_action_info::UserCouponActionInfo;
pub use user_coupon_expire_message::UserCouponExpireMessage;
pub use user_coupon_use_message::UserCouponUseMessage;
