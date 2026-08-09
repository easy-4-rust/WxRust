//! 视频号小店服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxChannelServiceImpl`（继承
//! `WxChannelServiceHttpComponentsImpl` → `BaseWxChannelServiceImpl`）：组合门面
//! trait 的默认实现 + 单配置存储（对应 Java `setConfig(WxChannelConfig)`）。
//! 子服务以 `Weak<dyn WxChannelService>` 注入（对应 Java
//! `new WxChannelBasicServiceImpl(this)` 的循环引用，Rust 用弱引用打破），
//! 由 `new_arc` 一次性装配 25 个子服务（对应 Java Base 构造器中的子服务字段
//! 与懒加载 getter：Java 前 12 个为 eager 字段、后 13 个为 synchronized
//! null 检查懒加载；Rust 以 `OnceLock<SubServices>` 构建时一次装配 +
//! getter 恒返回同一实例，语义等价）。
//!
//! 消息服务（`WxChannelMessageServiceImpl`）不在门面内：Java
//! `WxChannelService` 接口无 `getMessageService`，消息服务为独立子系统
//! （`BaseWxChannelMessageServiceImpl`），由调用方自行构建。

use std::sync::{Arc, OnceLock, RwLock};

use crate::api::r#impl::{
    WxAssistantServiceImpl, WxChannelAddressServiceImpl, WxChannelAfterSaleServiceImpl,
    WxChannelBasicServiceImpl, WxChannelBrandServiceImpl, WxChannelCategoryServiceImpl,
    WxChannelCompassFinderServiceImpl, WxChannelCompassShopServiceImpl, WxChannelCouponServiceImpl,
    WxChannelFreightTemplateServiceImpl, WxChannelFundServiceImpl,
    WxChannelLiveDashboardServiceImpl, WxChannelOrderServiceImpl, WxChannelProductServiceImpl,
    WxChannelSharerServiceImpl, WxChannelVipServiceImpl, WxChannelWarehouseServiceImpl,
    WxFinderLiveServiceImpl, WxLeadComponentServiceImpl, WxLeagueProductServiceImpl,
    WxLeaguePromoterServiceImpl, WxLeagueSupplierServiceImpl, WxLeagueWindowServiceImpl,
    WxStoreCooperationServiceImpl, WxStoreHomePageServiceImpl,
};
use crate::api::{
    WxAssistantService, WxChannelAddressService, WxChannelAfterSaleService, WxChannelBasicService,
    WxChannelBrandService, WxChannelCategoryService, WxChannelCompassFinderService,
    WxChannelCompassShopService, WxChannelCouponService, WxChannelFreightTemplateService,
    WxChannelFundService, WxChannelLiveDashboardService, WxChannelOrderService,
    WxChannelProductService, WxChannelService, WxChannelSharerService, WxChannelVipService,
    WxChannelWarehouseService, WxFinderLiveService, WxLeadComponentService, WxLeagueProductService,
    WxLeaguePromoterService, WxLeagueSupplierService, WxLeagueWindowService,
    WxStoreCooperationService, WxStoreHomePageService,
};
use crate::config::WxChannelConfig;

/// 子服务集合（对应 Java BaseWxChannelServiceImpl 构造器中实例化的 25 个子服务
/// 字段：basic/category/brand/product/warehouse/order/after_sale/freight_template/
/// address/coupon/sharer/fund/home_page/cooperation/compass_shop/league_window/
/// league_supplier/league_promoter/league_product/lead_component/finder_live/
/// assistant/vip/compass_finder/live_dashboard）。
struct SubServices {
    basic: Arc<dyn WxChannelBasicService>,
    category: Arc<dyn WxChannelCategoryService>,
    brand: Arc<dyn WxChannelBrandService>,
    product: Arc<dyn WxChannelProductService>,
    warehouse: Arc<dyn WxChannelWarehouseService>,
    order: Arc<dyn WxChannelOrderService>,
    after_sale: Arc<dyn WxChannelAfterSaleService>,
    freight_template: Arc<dyn WxChannelFreightTemplateService>,
    address: Arc<dyn WxChannelAddressService>,
    coupon: Arc<dyn WxChannelCouponService>,
    sharer: Arc<dyn WxChannelSharerService>,
    fund: Arc<dyn WxChannelFundService>,
    home_page: Arc<dyn WxStoreHomePageService>,
    cooperation: Arc<dyn WxStoreCooperationService>,
    compass_shop: Arc<dyn WxChannelCompassShopService>,
    league_window: Arc<dyn WxLeagueWindowService>,
    league_supplier: Arc<dyn WxLeagueSupplierService>,
    league_promoter: Arc<dyn WxLeaguePromoterService>,
    league_product: Arc<dyn WxLeagueProductService>,
    lead_component: Arc<dyn WxLeadComponentService>,
    finder_live: Arc<dyn WxFinderLiveService>,
    assistant: Arc<dyn WxAssistantService>,
    vip: Arc<dyn WxChannelVipService>,
    compass_finder: Arc<dyn WxChannelCompassFinderService>,
    live_dashboard: Arc<dyn WxChannelLiveDashboardService>,
}

/// 视频号小店服务实现（reqwest HTTP 后端）。
pub struct WxChannelServiceImpl {
    client: reqwest::Client,
    config: RwLock<Arc<dyn WxChannelConfig>>,
    sub_services: OnceLock<SubServices>,
}

impl WxChannelServiceImpl {
    /// 构建服务（子服务注入 `Weak<dyn WxChannelService>` 打破循环引用）。
    ///
    /// # 参数
    /// - `config`：视频号小店配置
    pub fn new_arc(config: Arc<dyn WxChannelConfig>) -> Arc<Self> {
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config: RwLock::new(config),
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxChannelService> 再降级为 Weak<dyn WxChannelService>
        // （对应 Java `new WxChannelXxxServiceImpl(this)` 的循环引用）
        let dyn_arc: Arc<dyn WxChannelService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            basic: Arc::new(WxChannelBasicServiceImpl::new(weak.clone())),
            category: Arc::new(WxChannelCategoryServiceImpl::new(weak.clone())),
            brand: Arc::new(WxChannelBrandServiceImpl::new(weak.clone())),
            product: Arc::new(WxChannelProductServiceImpl::new(weak.clone())),
            warehouse: Arc::new(WxChannelWarehouseServiceImpl::new(weak.clone())),
            order: Arc::new(WxChannelOrderServiceImpl::new(weak.clone())),
            after_sale: Arc::new(WxChannelAfterSaleServiceImpl::new(weak.clone())),
            freight_template: Arc::new(WxChannelFreightTemplateServiceImpl::new(weak.clone())),
            address: Arc::new(WxChannelAddressServiceImpl::new(weak.clone())),
            coupon: Arc::new(WxChannelCouponServiceImpl::new(weak.clone())),
            sharer: Arc::new(WxChannelSharerServiceImpl::new(weak.clone())),
            fund: Arc::new(WxChannelFundServiceImpl::new(weak.clone())),
            home_page: Arc::new(WxStoreHomePageServiceImpl::new(weak.clone())),
            cooperation: Arc::new(WxStoreCooperationServiceImpl::new(weak.clone())),
            compass_shop: Arc::new(WxChannelCompassShopServiceImpl::new(weak.clone())),
            league_window: Arc::new(WxLeagueWindowServiceImpl::new(weak.clone())),
            league_supplier: Arc::new(WxLeagueSupplierServiceImpl::new(weak.clone())),
            league_promoter: Arc::new(WxLeaguePromoterServiceImpl::new(weak.clone())),
            league_product: Arc::new(WxLeagueProductServiceImpl::new(weak.clone())),
            lead_component: Arc::new(WxLeadComponentServiceImpl::new(weak.clone())),
            finder_live: Arc::new(WxFinderLiveServiceImpl::new(weak.clone())),
            assistant: Arc::new(WxAssistantServiceImpl::new(weak.clone())),
            vip: Arc::new(WxChannelVipServiceImpl::new(weak.clone())),
            compass_finder: Arc::new(WxChannelCompassFinderServiceImpl::new(weak.clone())),
            live_dashboard: Arc::new(WxChannelLiveDashboardServiceImpl::new(weak.clone())),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &SubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }
}

impl WxChannelService for WxChannelServiceImpl {
    fn wx_channel_config(&self) -> Arc<dyn WxChannelConfig> {
        self.config.read().unwrap().clone()
    }

    fn set_config(&self, config: Arc<dyn WxChannelConfig>) {
        *self.config.write().unwrap() = config;
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// 基础接口服务（对应 Java `getBasicService()`）。
    fn basic_service(&self) -> Option<Arc<dyn WxChannelBasicService>> {
        Some(self.services().basic.clone())
    }

    /// 商品类目服务（对应 Java `getCategoryService()`）。
    fn category_service(&self) -> Option<Arc<dyn WxChannelCategoryService>> {
        Some(self.services().category.clone())
    }

    /// 品牌服务（对应 Java `getBrandService()`）。
    fn brand_service(&self) -> Option<Arc<dyn WxChannelBrandService>> {
        Some(self.services().brand.clone())
    }

    /// 商品服务（对应 Java `getProductService()`）。
    fn product_service(&self) -> Option<Arc<dyn WxChannelProductService>> {
        Some(self.services().product.clone())
    }

    /// 仓库服务（对应 Java `getWarehouseService()`）。
    fn warehouse_service(&self) -> Option<Arc<dyn WxChannelWarehouseService>> {
        Some(self.services().warehouse.clone())
    }

    /// 订单服务（对应 Java `getOrderService()`）。
    fn order_service(&self) -> Option<Arc<dyn WxChannelOrderService>> {
        Some(self.services().order.clone())
    }

    /// 售后服务（对应 Java `getAfterSaleService()`）。
    fn after_sale_service(&self) -> Option<Arc<dyn WxChannelAfterSaleService>> {
        Some(self.services().after_sale.clone())
    }

    /// 运费模板服务（对应 Java `getFreightTemplateService()`）。
    fn freight_template_service(&self) -> Option<Arc<dyn WxChannelFreightTemplateService>> {
        Some(self.services().freight_template.clone())
    }

    /// 地址服务（对应 Java `getAddressService()`）。
    fn address_service(&self) -> Option<Arc<dyn WxChannelAddressService>> {
        Some(self.services().address.clone())
    }

    /// 优惠券服务（对应 Java `getCouponService()`）。
    fn coupon_service(&self) -> Option<Arc<dyn WxChannelCouponService>> {
        Some(self.services().coupon.clone())
    }

    /// 分享员服务（对应 Java `getSharerService()`）。
    fn sharer_service(&self) -> Option<Arc<dyn WxChannelSharerService>> {
        Some(self.services().sharer.clone())
    }

    /// 资金服务（对应 Java `getFundService()`）。
    fn fund_service(&self) -> Option<Arc<dyn WxChannelFundService>> {
        Some(self.services().fund.clone())
    }

    /// 主页管理服务（对应 Java `getHomePageService()`，synchronized 懒加载）。
    fn home_page_service(&self) -> Option<Arc<dyn WxStoreHomePageService>> {
        Some(self.services().home_page.clone())
    }

    /// 合作账号服务（对应 Java `getCooperationService()`，synchronized 懒加载）。
    fn cooperation_service(&self) -> Option<Arc<dyn WxStoreCooperationService>> {
        Some(self.services().cooperation.clone())
    }

    /// 罗盘商家版服务（对应 Java `getCompassShopService()`，synchronized 懒加载）。
    fn compass_shop_service(&self) -> Option<Arc<dyn WxChannelCompassShopService>> {
        Some(self.services().compass_shop.clone())
    }

    /// 优选联盟-团长合作达人管理服务（对应 Java `getLeagueWindowService()`，
    /// synchronized 懒加载）。
    fn league_window_service(&self) -> Option<Arc<dyn WxLeagueWindowService>> {
        Some(self.services().league_window.clone())
    }

    /// 优选联盟-团长服务（对应 Java `getLeagueSupplierService()`，synchronized 懒加载）。
    fn league_supplier_service(&self) -> Option<Arc<dyn WxLeagueSupplierService>> {
        Some(self.services().league_supplier.clone())
    }

    /// 优选联盟-达人服务（对应 Java `getLeaguePromoterService()`，synchronized 懒加载）。
    fn league_promoter_service(&self) -> Option<Arc<dyn WxLeaguePromoterService>> {
        Some(self.services().league_promoter.clone())
    }

    /// 优选联盟-商品服务（对应 Java `getLeagueProductService()`，synchronized 懒加载）。
    fn league_product_service(&self) -> Option<Arc<dyn WxLeagueProductService>> {
        Some(self.services().league_product.clone())
    }

    /// 留资组件管理服务（对应 Java `getLeadComponentService()`，synchronized 懒加载）。
    fn lead_component_service(&self) -> Option<Arc<dyn WxLeadComponentService>> {
        Some(self.services().lead_component.clone())
    }

    /// 留资服务的直播数据服务（对应 Java `getFinderLiveService()`，synchronized 懒加载）。
    fn finder_live_service(&self) -> Option<Arc<dyn WxFinderLiveService>> {
        Some(self.services().finder_live.clone())
    }

    /// 视频号助手 橱窗管理服务（对应 Java `getAssistantService()`，synchronized 懒加载）。
    fn assistant_service(&self) -> Option<Arc<dyn WxAssistantService>> {
        Some(self.services().assistant.clone())
    }

    /// 会员服务（对应 Java `getVipService()`，synchronized 懒加载）。
    fn vip_service(&self) -> Option<Arc<dyn WxChannelVipService>> {
        Some(self.services().vip.clone())
    }

    /// 罗盘达人版服务（对应 Java `getCompassFinderService()`，synchronized 懒加载）。
    fn compass_finder_service(&self) -> Option<Arc<dyn WxChannelCompassFinderService>> {
        Some(self.services().compass_finder.clone())
    }

    /// 直播大屏数据服务（对应 Java `getLiveDashboardService()`，synchronized 懒加载）。
    fn live_dashboard_service(&self) -> Option<Arc<dyn WxChannelLiveDashboardService>> {
        Some(self.services().live_dashboard.clone())
    }
}
