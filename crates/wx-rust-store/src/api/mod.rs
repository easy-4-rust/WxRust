//! 微信小店 API。
//!
//! 对应 Java `com.binarywang.wxjava.store.api` 包。

pub mod r#impl;
pub mod wx_store_service;
pub use wx_store_service::WxStoreService;

pub mod wx_store_message_service;
pub use wx_store_message_service::WxStoreMessageService;
// 消息服务实现：已由 Wave 3 收尾并入 `impl/mod.rs` 统一注册
// （模块路径 `crate::api::r#impl::wx_store_message_service_impl`），
// 此处仅重导出保持公共 API 不变。
pub use r#impl::wx_store_message_service_impl::WxStoreMessageServiceImpl;

pub mod wx_store_basic_service;
pub use wx_store_basic_service::WxStoreBasicService;
pub mod wx_store_category_service;
pub use wx_store_category_service::WxStoreCategoryService;
pub mod wx_store_brand_service;
pub use wx_store_brand_service::WxStoreBrandService;
pub mod wx_store_product_service;
pub use wx_store_product_service::WxStoreProductService;
pub mod wx_store_warehouse_service;
pub use wx_store_warehouse_service::WxStoreWarehouseService;
pub mod wx_store_order_service;
pub use wx_store_order_service::WxStoreOrderService;
pub mod wx_store_after_sale_service;
pub use wx_store_after_sale_service::WxStoreAfterSaleService;
pub mod wx_store_freight_template_service;
pub use wx_store_freight_template_service::WxStoreFreightTemplateService;
pub mod wx_store_address_service;
pub use wx_store_address_service::WxStoreAddressService;
pub mod wx_store_coupon_service;
pub use wx_store_coupon_service::WxStoreCouponService;
pub mod wx_store_sharer_service;
pub use wx_store_sharer_service::WxStoreSharerService;
pub mod wx_store_fund_service;
pub use wx_store_fund_service::WxStoreFundService;
pub mod wx_store_home_page_service;
pub use wx_store_home_page_service::WxStoreHomePageService;
pub mod wx_store_cooperation_service;
pub use wx_store_cooperation_service::WxStoreCooperationService;
pub mod wx_store_compass_shop_service;
pub use wx_store_compass_shop_service::WxStoreCompassShopService;
pub mod wx_store_vip_service;
pub use wx_store_vip_service::WxStoreVipService;
pub mod wx_store_ewaybill_service;
pub use wx_store_ewaybill_service::WxStoreEwaybillService;
pub mod wx_store_favorite_service;
pub use wx_store_favorite_service::WxStoreFavoriteService;
pub mod wx_store_gift_service;
pub use wx_store_gift_service::WxStoreGiftService;
pub mod wx_store_kf_service;
pub use wx_store_kf_service::WxStoreKfService;
pub mod wx_store_limited_discount_service;
pub use wx_store_limited_discount_service::WxStoreLimitedDiscountService;
pub mod wx_store_product_assistant_service;
pub use wx_store_product_assistant_service::WxStoreProductAssistantService;
pub mod wx_store_product_stock_service;
pub use wx_store_product_stock_service::WxStoreProductStockService;
pub mod wx_store_qic_service;
pub use wx_store_qic_service::WxStoreQicService;
pub mod wx_store_supplier_service;
pub use wx_store_supplier_service::WxStoreSupplierService;
pub mod wx_talent_service;
pub use wx_talent_service::WxTalentService;
