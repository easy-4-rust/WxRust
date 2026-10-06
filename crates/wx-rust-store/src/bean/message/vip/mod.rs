//! 微信小店 会员 回调消息。
//!
//! 对应 Java `com.binarywang.wxjava.store.bean.message.vip` 包。

pub mod coupon_info;
pub mod exchange_info;
pub mod exchange_info_message;
pub mod product_info;
pub mod user_info;
pub mod user_info_message;

pub use coupon_info::CouponInfo;
pub use exchange_info::ExchangeInfo;
pub use exchange_info_message::ExchangeInfoMessage;
pub use product_info::ProductInfo;
pub use user_info::UserInfo;
pub use user_info_message::UserInfoMessage;
