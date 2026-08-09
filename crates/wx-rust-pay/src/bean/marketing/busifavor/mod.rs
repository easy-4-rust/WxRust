//! 对应 Java `com.github.binarywang.wxpay.bean.marketing/busifavor` 包（生成）。

pub mod available_week;
pub mod coupon_available_time;
pub mod coupon_use_rule;
pub mod custom_entrance;
pub mod discount_coupon;
pub mod display_pattern_info;
pub mod exchange_coupon;
pub mod fixed_normal_coupon;
pub mod irregulary_avaliable_time;
pub mod notify_config;
pub mod stock_send_rule;

pub use available_week::AvailableDayTimeItem;
pub use available_week::AvailableWeek;
pub use coupon_available_time::CouponAvailableTime;
pub use coupon_use_rule::CouponUseRule;
pub use custom_entrance::CustomEntrance;
pub use custom_entrance::MiniProgramsInfo;
pub use discount_coupon::DiscountCoupon;
pub use display_pattern_info::DisplayPatternInfo;
pub use exchange_coupon::ExchangeCoupon;
pub use fixed_normal_coupon::FixedNormalCoupon;
pub use irregulary_avaliable_time::IrregularyAvaliableTime;
pub use notify_config::NotifyConfig;
pub use stock_send_rule::StockSendRule;
