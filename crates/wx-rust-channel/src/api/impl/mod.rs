//! 视频号小店服务实现（对应 Java `me.chanjar.weixin.channel.api.impl` 包）。

pub mod base_wx_channel_service_impl;
pub mod wx_channel_address_service_impl;
pub mod wx_channel_after_sale_service_impl;
pub mod wx_channel_basic_service_impl;
pub mod wx_channel_brand_service_impl;
pub mod wx_channel_category_service_impl;
pub mod wx_channel_coupon_service_impl;
pub mod wx_channel_freight_template_service_impl;
pub mod wx_channel_order_service_impl;
pub mod wx_channel_product_service_impl;
pub mod wx_channel_service_impl;
pub mod wx_channel_sharer_service_impl;
pub mod wx_channel_warehouse_service_impl;
// H2b 组注册（Wave 2 H2b 产物）：非 shop 域 14 个子服务实现
// （fund/home_page/cooperation/compass_shop/league_window/league_supplier/
// league_promoter/league_product/lead_component/finder_live/assistant/vip/
// compass_finder/live_dashboard），子模块以 `#[path]` 指回本目录文件。
pub mod h2b_impls;
// H2c 消息服务实现（Wave 2 H2c 产物，原由 api/mod.rs `#[path]` 临时注册，
// Wave 3 收尾并入本模块统一注册）。
pub mod wx_channel_message_service_impl;

pub use h2b_impls::*;
pub use wx_channel_address_service_impl::WxChannelAddressServiceImpl;
pub use wx_channel_after_sale_service_impl::WxChannelAfterSaleServiceImpl;
pub use wx_channel_basic_service_impl::WxChannelBasicServiceImpl;
pub use wx_channel_brand_service_impl::WxChannelBrandServiceImpl;
pub use wx_channel_category_service_impl::WxChannelCategoryServiceImpl;
pub use wx_channel_coupon_service_impl::WxChannelCouponServiceImpl;
pub use wx_channel_freight_template_service_impl::WxChannelFreightTemplateServiceImpl;
pub use wx_channel_message_service_impl::WxChannelMessageServiceImpl;
pub use wx_channel_order_service_impl::WxChannelOrderServiceImpl;
pub use wx_channel_product_service_impl::WxChannelProductServiceImpl;
pub use wx_channel_service_impl::WxChannelServiceImpl;
pub use wx_channel_sharer_service_impl::WxChannelSharerServiceImpl;
pub use wx_channel_warehouse_service_impl::WxChannelWarehouseServiceImpl;

// 子服务实现注册（Wave 2 H2a：shop 域 11 个子服务实现批次，对应 Java
// `BaseWxChannelServiceImpl` 构造器中的子服务字段；装配见 Wave 3 门面）。
// 非 shop 域 14 个 + 消息服务 1 个见上方 h2b_impls / wx_channel_message_service_impl。
