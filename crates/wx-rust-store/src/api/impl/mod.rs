//! 微信小店服务实现（对应 Java `com.binarywang.wxjava.store.api.impl` 包）。

pub mod base_wx_store_service_impl;
pub mod wx_store_address_service_impl;
pub mod wx_store_after_sale_service_impl;
pub mod wx_store_basic_service_impl;
pub mod wx_store_brand_service_impl;
pub mod wx_store_category_service_impl;
pub mod wx_store_coupon_service_impl;
pub mod wx_store_freight_template_service_impl;
pub mod wx_store_order_service_impl;
pub mod wx_store_product_service_impl;
pub mod wx_store_service_impl;
pub mod wx_store_sharer_service_impl;
pub mod wx_store_warehouse_service_impl;
// H2b 组注册（Wave 2 H2b 产物）：非 shop 域 14 个子服务实现
// （fund/home_page/cooperation/compass_shop/league_window/league_supplier/
// league_promoter/league_product/lead_component/finder_live/assistant/vip/
// compass_finder/live_dashboard），子模块以 `#[path]` 指回本目录文件。
// H2c 消息服务实现（Wave 2 H2c 产物，原由 api/mod.rs `#[path]` 临时注册，
// Wave 3 收尾并入本模块统一注册）。
pub mod wx_store_ewaybill_service_impl;
pub mod wx_store_favorite_service_impl;
pub mod wx_store_gift_service_impl;
pub mod wx_store_kf_service_impl;
pub mod wx_store_limited_discount_service_impl;
pub mod wx_store_message_service_impl;
pub mod wx_store_product_assistant_service_impl;
pub mod wx_store_product_stock_service_impl;
pub mod wx_store_qic_service_impl;
pub mod wx_store_supplier_service_impl;
pub mod wx_talent_service_impl;

pub use wx_store_address_service_impl::WxStoreAddressServiceImpl;
pub use wx_store_after_sale_service_impl::WxStoreAfterSaleServiceImpl;
pub use wx_store_basic_service_impl::WxStoreBasicServiceImpl;
pub use wx_store_brand_service_impl::WxStoreBrandServiceImpl;
pub use wx_store_category_service_impl::WxStoreCategoryServiceImpl;
pub use wx_store_coupon_service_impl::WxStoreCouponServiceImpl;
pub use wx_store_ewaybill_service_impl::WxStoreEwaybillServiceImpl;
pub use wx_store_favorite_service_impl::WxStoreFavoriteServiceImpl;
pub use wx_store_freight_template_service_impl::WxStoreFreightTemplateServiceImpl;
pub use wx_store_gift_service_impl::WxStoreGiftServiceImpl;
pub use wx_store_kf_service_impl::WxStoreKfServiceImpl;
pub use wx_store_limited_discount_service_impl::WxStoreLimitedDiscountServiceImpl;
pub use wx_store_message_service_impl::WxStoreMessageServiceImpl;
pub use wx_store_order_service_impl::WxStoreOrderServiceImpl;
pub use wx_store_product_assistant_service_impl::WxStoreProductAssistantServiceImpl;
pub use wx_store_product_service_impl::WxStoreProductServiceImpl;
pub use wx_store_product_stock_service_impl::WxStoreProductStockServiceImpl;
pub use wx_store_qic_service_impl::WxStoreQicServiceImpl;
pub use wx_store_service_impl::WxStoreServiceImpl;
pub use wx_store_sharer_service_impl::WxStoreSharerServiceImpl;
pub use wx_store_supplier_service_impl::WxStoreSupplierServiceImpl;
pub use wx_store_warehouse_service_impl::WxStoreWarehouseServiceImpl;
pub use wx_talent_service_impl::WxTalentServiceImpl;

// 子服务实现注册（Wave 2 H2a：shop 域 11 个子服务实现批次，对应 Java
// `BaseWxStoreServiceImpl` 构造器中的子服务字段；装配见 Wave 3 门面）。
// 非 shop 域 14 个 + 消息服务 1 个见上方 h2b_impls / wx_store_message_service_impl。
pub mod wx_store_compass_shop_service_impl;
pub use wx_store_compass_shop_service_impl::WxStoreCompassShopServiceImpl;
pub mod wx_store_cooperation_service_impl;
pub use wx_store_cooperation_service_impl::WxStoreCooperationServiceImpl;
pub mod wx_store_fund_service_impl;
pub use wx_store_fund_service_impl::WxStoreFundServiceImpl;
pub mod wx_store_home_page_service_impl;
pub use wx_store_home_page_service_impl::WxStoreHomePageServiceImpl;
pub mod wx_store_vip_service_impl;
pub use wx_store_vip_service_impl::WxStoreVipServiceImpl;
#[cfg(test)]
mod h2b_impls;
