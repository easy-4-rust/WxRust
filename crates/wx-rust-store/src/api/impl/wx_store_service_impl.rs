use crate::api::WxStoreService;
use crate::api::r#impl::{
    WxStoreAddressServiceImpl, WxStoreAfterSaleServiceImpl, WxStoreBasicServiceImpl,
    WxStoreBrandServiceImpl, WxStoreCategoryServiceImpl, WxStoreCompassShopServiceImpl,
    WxStoreCooperationServiceImpl, WxStoreCouponServiceImpl, WxStoreEwaybillServiceImpl,
    WxStoreFavoriteServiceImpl, WxStoreFreightTemplateServiceImpl, WxStoreFundServiceImpl,
    WxStoreGiftServiceImpl, WxStoreHomePageServiceImpl, WxStoreKfServiceImpl,
    WxStoreLimitedDiscountServiceImpl, WxStoreOrderServiceImpl, WxStoreProductAssistantServiceImpl,
    WxStoreProductServiceImpl, WxStoreProductStockServiceImpl, WxStoreQicServiceImpl,
    WxStoreSharerServiceImpl, WxStoreSupplierServiceImpl, WxStoreVipServiceImpl,
    WxStoreWarehouseServiceImpl, WxTalentServiceImpl,
};
use crate::api::{
    WxStoreAddressService, WxStoreAfterSaleService, WxStoreBasicService, WxStoreBrandService,
    WxStoreCategoryService, WxStoreCompassShopService, WxStoreCooperationService,
    WxStoreCouponService, WxStoreEwaybillService, WxStoreFavoriteService,
    WxStoreFreightTemplateService, WxStoreFundService, WxStoreGiftService, WxStoreHomePageService,
    WxStoreKfService, WxStoreLimitedDiscountService, WxStoreOrderService,
    WxStoreProductAssistantService, WxStoreProductService, WxStoreProductStockService,
    WxStoreQicService, WxStoreSharerService, WxStoreSupplierService, WxStoreVipService,
    WxStoreWarehouseService, WxTalentService,
};
use crate::config::WxStoreConfig;
use std::sync::{Arc, OnceLock, RwLock};

/// 微信小店服务集合，对应 Java BaseWxStoreServiceImpl 的子服务字段。
struct SubServices {
    address: Arc<dyn WxStoreAddressService>,
    after_sale: Arc<dyn WxStoreAfterSaleService>,
    basic: Arc<dyn WxStoreBasicService>,
    brand: Arc<dyn WxStoreBrandService>,
    category: Arc<dyn WxStoreCategoryService>,
    compass_shop: Arc<dyn WxStoreCompassShopService>,
    cooperation: Arc<dyn WxStoreCooperationService>,
    coupon: Arc<dyn WxStoreCouponService>,
    ewaybill: Arc<dyn WxStoreEwaybillService>,
    favorite: Arc<dyn WxStoreFavoriteService>,
    freight_template: Arc<dyn WxStoreFreightTemplateService>,
    fund: Arc<dyn WxStoreFundService>,
    gift: Arc<dyn WxStoreGiftService>,
    home_page: Arc<dyn WxStoreHomePageService>,
    kf: Arc<dyn WxStoreKfService>,
    limited_discount: Arc<dyn WxStoreLimitedDiscountService>,
    order: Arc<dyn WxStoreOrderService>,
    product_assistant: Arc<dyn WxStoreProductAssistantService>,
    product: Arc<dyn WxStoreProductService>,
    product_stock: Arc<dyn WxStoreProductStockService>,
    qic: Arc<dyn WxStoreQicService>,
    sharer: Arc<dyn WxStoreSharerService>,
    supplier: Arc<dyn WxStoreSupplierService>,
    vip: Arc<dyn WxStoreVipService>,
    warehouse: Arc<dyn WxStoreWarehouseService>,
    talent: Arc<dyn WxTalentService>,
}
/// 独立微信小店服务实现。
/// 对应 Java: com.binarywang.wxjava.store.api.impl.WxStoreServiceImpl
pub struct WxStoreServiceImpl {
    client: reqwest::Client,
    config: RwLock<Arc<dyn WxStoreConfig>>,
    services: OnceLock<SubServices>,
}
impl WxStoreServiceImpl {
    /// 使用独立店铺配置构建并返回已装配全部经营子服务的共享客户端。
    /// 对应 Java: BaseWxStoreServiceImpl#setConfig
    pub fn new_arc(config: Arc<dyn WxStoreConfig>) -> Arc<Self> {
        let service = Arc::new(Self {
            client: reqwest::Client::new(),
            config: RwLock::new(config),
            services: OnceLock::new(),
        });
        let erased: Arc<dyn WxStoreService> = service.clone();
        let weak = Arc::downgrade(&erased);
        let _ = service.services.set(SubServices {
            address: Arc::new(WxStoreAddressServiceImpl::new(weak.clone())),
            after_sale: Arc::new(WxStoreAfterSaleServiceImpl::new(weak.clone())),
            basic: Arc::new(WxStoreBasicServiceImpl::new(weak.clone())),
            brand: Arc::new(WxStoreBrandServiceImpl::new(weak.clone())),
            category: Arc::new(WxStoreCategoryServiceImpl::new(weak.clone())),
            compass_shop: Arc::new(WxStoreCompassShopServiceImpl::new(weak.clone())),
            cooperation: Arc::new(WxStoreCooperationServiceImpl::new(weak.clone())),
            coupon: Arc::new(WxStoreCouponServiceImpl::new(weak.clone())),
            ewaybill: Arc::new(WxStoreEwaybillServiceImpl::new(weak.clone())),
            favorite: Arc::new(WxStoreFavoriteServiceImpl::new(weak.clone())),
            freight_template: Arc::new(WxStoreFreightTemplateServiceImpl::new(weak.clone())),
            fund: Arc::new(WxStoreFundServiceImpl::new(weak.clone())),
            gift: Arc::new(WxStoreGiftServiceImpl::new(weak.clone())),
            home_page: Arc::new(WxStoreHomePageServiceImpl::new(weak.clone())),
            kf: Arc::new(WxStoreKfServiceImpl::new(weak.clone())),
            limited_discount: Arc::new(WxStoreLimitedDiscountServiceImpl::new(weak.clone())),
            order: Arc::new(WxStoreOrderServiceImpl::new(weak.clone())),
            product_assistant: Arc::new(WxStoreProductAssistantServiceImpl::new(weak.clone())),
            product: Arc::new(WxStoreProductServiceImpl::new(weak.clone())),
            product_stock: Arc::new(WxStoreProductStockServiceImpl::new(weak.clone())),
            qic: Arc::new(WxStoreQicServiceImpl::new(weak.clone())),
            sharer: Arc::new(WxStoreSharerServiceImpl::new(weak.clone())),
            supplier: Arc::new(WxStoreSupplierServiceImpl::new(weak.clone())),
            vip: Arc::new(WxStoreVipServiceImpl::new(weak.clone())),
            warehouse: Arc::new(WxStoreWarehouseServiceImpl::new(weak.clone())),
            talent: Arc::new(WxTalentServiceImpl::new(weak.clone())),
        });
        service
    }
}
impl WxStoreService for WxStoreServiceImpl {
    fn wx_store_config(&self) -> Arc<dyn WxStoreConfig> {
        self.config.read().unwrap().clone()
    }
    fn set_config(&self, config: Arc<dyn WxStoreConfig>) {
        *self.config.write().unwrap() = config;
    }
    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }
    fn address_service(&self) -> Option<Arc<dyn WxStoreAddressService>> {
        Some(self.services.get().expect("子服务已初始化").address.clone())
    }
    fn after_sale_service(&self) -> Option<Arc<dyn WxStoreAfterSaleService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .after_sale
                .clone(),
        )
    }
    fn basic_service(&self) -> Option<Arc<dyn WxStoreBasicService>> {
        Some(self.services.get().expect("子服务已初始化").basic.clone())
    }
    fn brand_service(&self) -> Option<Arc<dyn WxStoreBrandService>> {
        Some(self.services.get().expect("子服务已初始化").brand.clone())
    }
    fn category_service(&self) -> Option<Arc<dyn WxStoreCategoryService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .category
                .clone(),
        )
    }
    fn compass_shop_service(&self) -> Option<Arc<dyn WxStoreCompassShopService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .compass_shop
                .clone(),
        )
    }
    fn cooperation_service(&self) -> Option<Arc<dyn WxStoreCooperationService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .cooperation
                .clone(),
        )
    }
    fn coupon_service(&self) -> Option<Arc<dyn WxStoreCouponService>> {
        Some(self.services.get().expect("子服务已初始化").coupon.clone())
    }
    fn ewaybill_service(&self) -> Option<Arc<dyn WxStoreEwaybillService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .ewaybill
                .clone(),
        )
    }
    fn favorite_service(&self) -> Option<Arc<dyn WxStoreFavoriteService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .favorite
                .clone(),
        )
    }
    fn freight_template_service(&self) -> Option<Arc<dyn WxStoreFreightTemplateService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .freight_template
                .clone(),
        )
    }
    fn fund_service(&self) -> Option<Arc<dyn WxStoreFundService>> {
        Some(self.services.get().expect("子服务已初始化").fund.clone())
    }
    fn gift_service(&self) -> Option<Arc<dyn WxStoreGiftService>> {
        Some(self.services.get().expect("子服务已初始化").gift.clone())
    }
    fn home_page_service(&self) -> Option<Arc<dyn WxStoreHomePageService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .home_page
                .clone(),
        )
    }
    fn kf_service(&self) -> Option<Arc<dyn WxStoreKfService>> {
        Some(self.services.get().expect("子服务已初始化").kf.clone())
    }
    fn limited_discount_service(&self) -> Option<Arc<dyn WxStoreLimitedDiscountService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .limited_discount
                .clone(),
        )
    }
    fn order_service(&self) -> Option<Arc<dyn WxStoreOrderService>> {
        Some(self.services.get().expect("子服务已初始化").order.clone())
    }
    fn product_assistant_service(&self) -> Option<Arc<dyn WxStoreProductAssistantService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .product_assistant
                .clone(),
        )
    }
    fn product_service(&self) -> Option<Arc<dyn WxStoreProductService>> {
        Some(self.services.get().expect("子服务已初始化").product.clone())
    }
    fn product_stock_service(&self) -> Option<Arc<dyn WxStoreProductStockService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .product_stock
                .clone(),
        )
    }
    fn qic_service(&self) -> Option<Arc<dyn WxStoreQicService>> {
        Some(self.services.get().expect("子服务已初始化").qic.clone())
    }
    fn sharer_service(&self) -> Option<Arc<dyn WxStoreSharerService>> {
        Some(self.services.get().expect("子服务已初始化").sharer.clone())
    }
    fn supplier_service(&self) -> Option<Arc<dyn WxStoreSupplierService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .supplier
                .clone(),
        )
    }
    fn vip_service(&self) -> Option<Arc<dyn WxStoreVipService>> {
        Some(self.services.get().expect("子服务已初始化").vip.clone())
    }
    fn warehouse_service(&self) -> Option<Arc<dyn WxStoreWarehouseService>> {
        Some(
            self.services
                .get()
                .expect("子服务已初始化")
                .warehouse
                .clone(),
        )
    }
    fn talent_service(&self) -> Option<Arc<dyn WxTalentService>> {
        Some(self.services.get().expect("子服务已初始化").talent.clone())
    }
}
