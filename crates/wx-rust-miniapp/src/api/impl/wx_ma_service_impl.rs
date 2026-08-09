//! 小程序服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaServiceImpl`（继承
//! `WxMaServiceHttpComponentsImpl` → `BaseWxMaServiceImpl`）：组合门面 trait
//! 的默认实现 + 多小程序配置管理（对应 Java `configMap` +
//! `WxMaConfigHolder`）。子服务以 `Weak<dyn WxMaService>` 注入（对应 Java
//! `new WxMaUserServiceImpl(this)` 的循环引用，Rust 用弱引用打破），由
//! Wave 3 一次性装配 53 个子服务（对应 Java Base 构造器中的子服务字段）。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::api::r#impl::{
    WxMaAnalysisServiceImpl, WxMaCloudServiceImpl, WxMaCodeServiceImpl, WxMaComplaintServiceImpl,
    WxMaCustomserviceWorkServiceImpl, WxMaDeviceSubscribeServiceImpl,
    WxMaEmployeeRelationServiceImpl, WxMaExpressDeliveryReturnServiceImpl, WxMaExpressServiceImpl,
    WxMaFaceServiceImpl, WxMaImgProcServiceImpl, WxMaImmediateDeliveryServiceImpl,
    WxMaInternetServiceImpl, WxMaIntracityServiceImpl, WxMaJsapiServiceImpl, WxMaKefuServiceImpl,
    WxMaLinkServiceImpl, WxMaLiveGoodsServiceImpl, WxMaLiveMemberServiceImpl, WxMaLiveServiceImpl,
    WxMaMarketingServiceImpl, WxMaMediaServiceImpl, WxMaMsgServiceImpl, WxMaOcrServiceImpl,
    WxMaOpenApiServiceImpl, WxMaOrderManagementServiceImpl, WxMaOrderShippingServiceImpl,
    WxMaPluginServiceImpl, WxMaProductOrderServiceImpl, WxMaProductServiceImpl,
    WxMaPromotionServiceImpl, WxMaQrcodeJumpServiceImpl, WxMaQrcodeServiceImpl,
    WxMaReimburseInvoiceServiceImpl, WxMaRunServiceImpl, WxMaSchemeServiceImpl,
    WxMaSecurityServiceImpl, WxMaSettingServiceImpl, WxMaShareServiceImpl,
    WxMaShopAccountServiceImpl, WxMaShopAfterSaleServiceImpl, WxMaShopAuditServiceImpl,
    WxMaShopCatServiceImpl, WxMaShopCouponServiceImpl, WxMaShopDeliveryServiceImpl,
    WxMaShopImgServiceImpl, WxMaShopOrderServiceImpl, WxMaShopPayServiceImpl,
    WxMaShopRegisterServiceImpl, WxMaShopSharerServiceImpl, WxMaShopSpuServiceImpl,
    WxMaSubscribeServiceImpl, WxMaUserServiceImpl, WxMaVodServiceImpl, WxMaXPayServiceImpl,
};
use crate::api::{
    WxMaAnalysisService, WxMaCloudService, WxMaCodeService, WxMaComplaintService,
    WxMaCustomserviceWorkService, WxMaDeviceSubscribeService, WxMaEmployeeRelationService,
    WxMaExpressDeliveryReturnService, WxMaExpressService, WxMaFaceService,
    WxMaImmediateDeliveryService, WxMaInternetService, WxMaIntracityService, WxMaJsapiService,
    WxMaKefuService, WxMaLinkService, WxMaLiveGoodsService, WxMaLiveMemberService, WxMaLiveService,
    WxMaMarketingService, WxMaMediaService, WxMaMsgService, WxMaOpenApiService,
    WxMaOrderManagementService, WxMaOrderShippingService, WxMaPluginService,
    WxMaProductOrderService, WxMaProductService, WxMaPromotionService, WxMaQrcodeJumpService,
    WxMaQrcodeService, WxMaReimburseInvoiceService, WxMaRunService, WxMaSchemeService,
    WxMaSecurityService, WxMaService, WxMaSettingService, WxMaShareService, WxMaShopAccountService,
    WxMaShopAfterSaleService, WxMaShopAuditService, WxMaShopCatService, WxMaShopCouponService,
    WxMaShopDeliveryService, WxMaShopImgService, WxMaShopOrderService, WxMaShopPayService,
    WxMaShopRegisterService, WxMaShopSharerService, WxMaShopSpuService, WxMaSubscribeService,
    WxMaUserService, WxMaVodService, WxMaXPayService,
};
use crate::config::WxMaConfig;
use crate::util::wx_ma_config_holder;
use wx_rust_common::service::{WxImgProcService, WxOcrService};

/// 子服务集合（对应 Java WxMaService 各 `getXxxService()` 返回的子服务字段）。
struct SubServices {
    user: Arc<dyn WxMaUserService>,
    msg: Arc<dyn WxMaMsgService>,
    media: Arc<dyn WxMaMediaService>,
    kefu: Arc<dyn WxMaKefuService>,
    analysis: Arc<dyn WxMaAnalysisService>,
    code: Arc<dyn WxMaCodeService>,
    express: Arc<dyn WxMaExpressService>,
    security: Arc<dyn WxMaSecurityService>,
    setting: Arc<dyn WxMaSettingService>,
    subscribe: Arc<dyn WxMaSubscribeService>,
    share: Arc<dyn WxMaShareService>,
    scheme: Arc<dyn WxMaSchemeService>,
    link: Arc<dyn WxMaLinkService>,
    qrcode: Arc<dyn WxMaQrcodeService>,
    jsapi: Arc<dyn WxMaJsapiService>,
    plugin: Arc<dyn WxMaPluginService>,
    run: Arc<dyn WxMaRunService>,
    open_api: Arc<dyn WxMaOpenApiService>,
    internet: Arc<dyn WxMaInternetService>,
    shop_account: Arc<dyn WxMaShopAccountService>,
    shop_after_sale: Arc<dyn WxMaShopAfterSaleService>,
    shop_audit: Arc<dyn WxMaShopAuditService>,
    shop_cat: Arc<dyn WxMaShopCatService>,
    shop_coupon: Arc<dyn WxMaShopCouponService>,
    shop_delivery: Arc<dyn WxMaShopDeliveryService>,
    shop_img: Arc<dyn WxMaShopImgService>,
    shop_order: Arc<dyn WxMaShopOrderService>,
    shop_pay: Arc<dyn WxMaShopPayService>,
    shop_register: Arc<dyn WxMaShopRegisterService>,
    shop_sharer: Arc<dyn WxMaShopSharerService>,
    shop_spu: Arc<dyn WxMaShopSpuService>,
    product: Arc<dyn WxMaProductService>,
    product_order: Arc<dyn WxMaProductOrderService>,
    order_management: Arc<dyn WxMaOrderManagementService>,
    order_shipping: Arc<dyn WxMaOrderShippingService>,
    express_delivery_return: Arc<dyn WxMaExpressDeliveryReturnService>,
    immediate_delivery: Arc<dyn WxMaImmediateDeliveryService>,
    employee_relation: Arc<dyn WxMaEmployeeRelationService>,
    customservice_work: Arc<dyn WxMaCustomserviceWorkService>,
    live: Arc<dyn WxMaLiveService>,
    live_goods: Arc<dyn WxMaLiveGoodsService>,
    live_member: Arc<dyn WxMaLiveMemberService>,
    cloud: Arc<dyn WxMaCloudService>,
    vod: Arc<dyn WxMaVodService>,
    xpay: Arc<dyn WxMaXPayService>,
    marketing: Arc<dyn WxMaMarketingService>,
    promotion: Arc<dyn WxMaPromotionService>,
    intracity: Arc<dyn WxMaIntracityService>,
    complaint: Arc<dyn WxMaComplaintService>,
    device_subscribe: Arc<dyn WxMaDeviceSubscribeService>,
    face: Arc<dyn WxMaFaceService>,
    reimburse_invoice: Arc<dyn WxMaReimburseInvoiceService>,
    qrcode_jump: Arc<dyn WxMaQrcodeJumpService>,
    // 对应 Java `WxMaService.getOcrService()/getImgProcService()`：无独立
    // 接口，直接实现 common trait（`WxOcrService`/`WxImgProcService`）
    ocr: Arc<dyn WxOcrService>,
    img_proc: Arc<dyn WxImgProcService>,
}

/// 小程序服务实现（reqwest HTTP 后端）。
pub struct WxMaServiceImpl {
    client: reqwest::Client,
    config_storages: Mutex<HashMap<String, Arc<dyn WxMaConfig>>>,
    default_appid: String,
    sub_services: OnceLock<SubServices>,
}

impl WxMaServiceImpl {
    /// 构建服务（子服务注入 `Weak<dyn WxMaService>` 打破循环引用）。
    ///
    /// # 参数
    /// - `config`：初始小程序配置
    pub fn new_arc(config: Arc<dyn WxMaConfig>) -> Arc<Self> {
        let appid = config.app_id().to_string();
        let mut storages = HashMap::new();
        storages.insert(appid.clone(), config);
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storages: Mutex::new(storages),
            default_appid: appid,
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxMaService> 再降级为 Weak<dyn WxMaService>
        let dyn_arc: Arc<dyn WxMaService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            user: Arc::new(WxMaUserServiceImpl::new(weak.clone())),
            msg: Arc::new(WxMaMsgServiceImpl::new(weak.clone())),
            media: Arc::new(WxMaMediaServiceImpl::new(weak.clone())),
            kefu: Arc::new(WxMaKefuServiceImpl::new(weak.clone())),
            analysis: Arc::new(WxMaAnalysisServiceImpl::new(weak.clone())),
            code: Arc::new(WxMaCodeServiceImpl::new(weak.clone())),
            express: Arc::new(WxMaExpressServiceImpl::new(weak.clone())),
            security: Arc::new(WxMaSecurityServiceImpl::new(weak.clone())),
            setting: Arc::new(WxMaSettingServiceImpl::new(weak.clone())),
            subscribe: Arc::new(WxMaSubscribeServiceImpl::new(weak.clone())),
            share: Arc::new(WxMaShareServiceImpl::new(weak.clone())),
            scheme: Arc::new(WxMaSchemeServiceImpl::new(weak.clone())),
            link: Arc::new(WxMaLinkServiceImpl::new(weak.clone())),
            qrcode: Arc::new(WxMaQrcodeServiceImpl::new(weak.clone())),
            jsapi: Arc::new(WxMaJsapiServiceImpl::new(weak.clone())),
            plugin: Arc::new(WxMaPluginServiceImpl::new(weak.clone())),
            run: Arc::new(WxMaRunServiceImpl::new(weak.clone())),
            open_api: Arc::new(WxMaOpenApiServiceImpl::new(weak.clone())),
            internet: Arc::new(WxMaInternetServiceImpl::new(weak.clone())),
            shop_account: Arc::new(WxMaShopAccountServiceImpl::new(weak.clone())),
            shop_after_sale: Arc::new(WxMaShopAfterSaleServiceImpl::new(weak.clone())),
            shop_audit: Arc::new(WxMaShopAuditServiceImpl::new(weak.clone())),
            shop_cat: Arc::new(WxMaShopCatServiceImpl::new(weak.clone())),
            shop_coupon: Arc::new(WxMaShopCouponServiceImpl::new(weak.clone())),
            shop_delivery: Arc::new(WxMaShopDeliveryServiceImpl::new(weak.clone())),
            shop_img: Arc::new(WxMaShopImgServiceImpl::new(weak.clone())),
            shop_order: Arc::new(WxMaShopOrderServiceImpl::new(weak.clone())),
            shop_pay: Arc::new(WxMaShopPayServiceImpl::new(weak.clone())),
            shop_register: Arc::new(WxMaShopRegisterServiceImpl::new(weak.clone())),
            shop_sharer: Arc::new(WxMaShopSharerServiceImpl::new(weak.clone())),
            shop_spu: Arc::new(WxMaShopSpuServiceImpl::new(weak.clone())),
            product: Arc::new(WxMaProductServiceImpl::new(weak.clone())),
            product_order: Arc::new(WxMaProductOrderServiceImpl::new(weak.clone())),
            order_management: Arc::new(WxMaOrderManagementServiceImpl::new(weak.clone())),
            order_shipping: Arc::new(WxMaOrderShippingServiceImpl::new(weak.clone())),
            express_delivery_return: Arc::new(WxMaExpressDeliveryReturnServiceImpl::new(
                weak.clone(),
            )),
            immediate_delivery: Arc::new(WxMaImmediateDeliveryServiceImpl::new(weak.clone())),
            employee_relation: Arc::new(WxMaEmployeeRelationServiceImpl::new(weak.clone())),
            customservice_work: Arc::new(WxMaCustomserviceWorkServiceImpl::new(weak.clone())),
            live: Arc::new(WxMaLiveServiceImpl::new(weak.clone())),
            live_goods: Arc::new(WxMaLiveGoodsServiceImpl::new(weak.clone())),
            live_member: Arc::new(WxMaLiveMemberServiceImpl::new(weak.clone())),
            cloud: Arc::new(WxMaCloudServiceImpl::new(weak.clone())),
            vod: Arc::new(WxMaVodServiceImpl::new(weak.clone())),
            xpay: Arc::new(WxMaXPayServiceImpl::new(weak.clone())),
            marketing: Arc::new(WxMaMarketingServiceImpl::new(weak.clone())),
            promotion: Arc::new(WxMaPromotionServiceImpl::new(weak.clone())),
            intracity: Arc::new(WxMaIntracityServiceImpl::new(weak.clone())),
            complaint: Arc::new(WxMaComplaintServiceImpl::new(weak.clone())),
            device_subscribe: Arc::new(WxMaDeviceSubscribeServiceImpl::new(weak.clone())),
            face: Arc::new(WxMaFaceServiceImpl::new(weak.clone())),
            reimburse_invoice: Arc::new(WxMaReimburseInvoiceServiceImpl::new(weak.clone())),
            qrcode_jump: Arc::new(WxMaQrcodeJumpServiceImpl::new(weak.clone())),
            ocr: Arc::new(WxMaOcrServiceImpl::new(weak.clone())),
            img_proc: Arc::new(WxMaImgProcServiceImpl::new(weak.clone())),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &SubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }

    /// 设置当前小程序配置（对应 Java `setWxMaConfig`：appid 不能为 null）。
    pub fn set_config_storage(&self, config: Arc<dyn WxMaConfig>) -> Result<(), String> {
        let appid = config.app_id();
        if appid.is_empty() {
            return Err("appid不能设置为null".to_string());
        }
        let mut map = self.config_storages.lock().unwrap();
        map.clear();
        map.insert(appid.to_string(), config);
        Ok(())
    }

    /// 动态添加小程序配置（对应 Java `addConfig`）。
    pub fn add_config_storage(&self, miniapp_id: &str, config: Arc<dyn WxMaConfig>) {
        let mut map = self.config_storages.lock().unwrap();
        map.insert(miniapp_id.to_string(), config);
        wx_ma_config_holder::set(miniapp_id);
    }

    /// 动态移除小程序配置（对应 Java `removeConfig`）。
    ///
    /// 移除的是当前持有器指向的配置时，自动将剩余首个配置设为默认。
    pub fn remove_config_storage(&self, miniapp_id: &str) {
        let mut map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            // Java：已删除最后一个小程序配置，须立即补充新配置
            map.remove(miniapp_id);
            return;
        }
        if wx_ma_config_holder::get() == miniapp_id {
            map.remove(miniapp_id);
            if let Some(next) = map.keys().next() {
                wx_ma_config_holder::set(next.clone());
            }
            return;
        }
        map.remove(miniapp_id);
    }

    /// 切换到指定小程序（对应 Java `switchoverTo`）。
    pub fn switchover_to(&self, miniapp_id: &str) -> Result<(), String> {
        let map = self.config_storages.lock().unwrap();
        if map.contains_key(miniapp_id) {
            wx_ma_config_holder::set(miniapp_id);
            Ok(())
        } else {
            Err(format!(
                "无法找到对应【{miniapp_id}】的小程序配置信息，请核实！"
            ))
        }
    }
}

impl WxMaService for WxMaServiceImpl {
    fn wx_ma_config(&self) -> Arc<dyn WxMaConfig> {
        let map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            // 只有一个小程序，直接返回其配置
            return map.values().next().unwrap().clone();
        }
        let holder = wx_ma_config_holder::get();
        map.get(&holder)
            .cloned()
            .or_else(|| map.get(&self.default_appid).cloned())
            .unwrap_or_else(|| map.values().next().unwrap().clone())
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn user_service(&self) -> Option<Arc<dyn WxMaUserService>> {
        Some(self.services().user.clone())
    }

    fn msg_service(&self) -> Option<Arc<dyn WxMaMsgService>> {
        Some(self.services().msg.clone())
    }

    fn media_service(&self) -> Option<Arc<dyn WxMaMediaService>> {
        Some(self.services().media.clone())
    }

    fn kefu_service(&self) -> Option<Arc<dyn WxMaKefuService>> {
        Some(self.services().kefu.clone())
    }

    fn analysis_service(&self) -> Option<Arc<dyn WxMaAnalysisService>> {
        Some(self.services().analysis.clone())
    }

    fn code_service(&self) -> Option<Arc<dyn WxMaCodeService>> {
        Some(self.services().code.clone())
    }

    fn express_service(&self) -> Option<Arc<dyn WxMaExpressService>> {
        Some(self.services().express.clone())
    }

    fn security_service(&self) -> Option<Arc<dyn WxMaSecurityService>> {
        Some(self.services().security.clone())
    }

    fn setting_service(&self) -> Option<Arc<dyn WxMaSettingService>> {
        Some(self.services().setting.clone())
    }

    fn subscribe_service(&self) -> Option<Arc<dyn WxMaSubscribeService>> {
        Some(self.services().subscribe.clone())
    }

    fn share_service(&self) -> Option<Arc<dyn WxMaShareService>> {
        Some(self.services().share.clone())
    }

    fn scheme_service(&self) -> Option<Arc<dyn WxMaSchemeService>> {
        Some(self.services().scheme.clone())
    }

    fn link_service(&self) -> Option<Arc<dyn WxMaLinkService>> {
        Some(self.services().link.clone())
    }

    fn qrcode_service(&self) -> Option<Arc<dyn WxMaQrcodeService>> {
        Some(self.services().qrcode.clone())
    }

    fn jsapi_service(&self) -> Option<Arc<dyn WxMaJsapiService>> {
        Some(self.services().jsapi.clone())
    }

    fn plugin_service(&self) -> Option<Arc<dyn WxMaPluginService>> {
        Some(self.services().plugin.clone())
    }

    fn run_service(&self) -> Option<Arc<dyn WxMaRunService>> {
        Some(self.services().run.clone())
    }

    fn open_api_service(&self) -> Option<Arc<dyn WxMaOpenApiService>> {
        Some(self.services().open_api.clone())
    }

    fn internet_service(&self) -> Option<Arc<dyn WxMaInternetService>> {
        Some(self.services().internet.clone())
    }

    fn shop_account_service(&self) -> Option<Arc<dyn WxMaShopAccountService>> {
        Some(self.services().shop_account.clone())
    }

    fn shop_after_sale_service(&self) -> Option<Arc<dyn WxMaShopAfterSaleService>> {
        Some(self.services().shop_after_sale.clone())
    }

    fn shop_audit_service(&self) -> Option<Arc<dyn WxMaShopAuditService>> {
        Some(self.services().shop_audit.clone())
    }

    fn shop_cat_service(&self) -> Option<Arc<dyn WxMaShopCatService>> {
        Some(self.services().shop_cat.clone())
    }

    fn shop_coupon_service(&self) -> Option<Arc<dyn WxMaShopCouponService>> {
        Some(self.services().shop_coupon.clone())
    }

    fn shop_delivery_service(&self) -> Option<Arc<dyn WxMaShopDeliveryService>> {
        Some(self.services().shop_delivery.clone())
    }

    fn shop_img_service(&self) -> Option<Arc<dyn WxMaShopImgService>> {
        Some(self.services().shop_img.clone())
    }

    fn shop_order_service(&self) -> Option<Arc<dyn WxMaShopOrderService>> {
        Some(self.services().shop_order.clone())
    }

    fn shop_pay_service(&self) -> Option<Arc<dyn WxMaShopPayService>> {
        Some(self.services().shop_pay.clone())
    }

    fn shop_register_service(&self) -> Option<Arc<dyn WxMaShopRegisterService>> {
        Some(self.services().shop_register.clone())
    }

    fn shop_sharer_service(&self) -> Option<Arc<dyn WxMaShopSharerService>> {
        Some(self.services().shop_sharer.clone())
    }

    fn shop_spu_service(&self) -> Option<Arc<dyn WxMaShopSpuService>> {
        Some(self.services().shop_spu.clone())
    }

    fn product_service(&self) -> Option<Arc<dyn WxMaProductService>> {
        Some(self.services().product.clone())
    }

    fn product_order_service(&self) -> Option<Arc<dyn WxMaProductOrderService>> {
        Some(self.services().product_order.clone())
    }

    fn order_management_service(&self) -> Option<Arc<dyn WxMaOrderManagementService>> {
        Some(self.services().order_management.clone())
    }

    fn order_shipping_service(&self) -> Option<Arc<dyn WxMaOrderShippingService>> {
        Some(self.services().order_shipping.clone())
    }

    fn express_delivery_return_service(&self) -> Option<Arc<dyn WxMaExpressDeliveryReturnService>> {
        Some(self.services().express_delivery_return.clone())
    }

    fn immediate_delivery_service(&self) -> Option<Arc<dyn WxMaImmediateDeliveryService>> {
        Some(self.services().immediate_delivery.clone())
    }

    fn employee_relation_service(&self) -> Option<Arc<dyn WxMaEmployeeRelationService>> {
        Some(self.services().employee_relation.clone())
    }

    fn customservice_work_service(&self) -> Option<Arc<dyn WxMaCustomserviceWorkService>> {
        Some(self.services().customservice_work.clone())
    }

    fn live_service(&self) -> Option<Arc<dyn WxMaLiveService>> {
        Some(self.services().live.clone())
    }

    fn live_goods_service(&self) -> Option<Arc<dyn WxMaLiveGoodsService>> {
        Some(self.services().live_goods.clone())
    }

    fn live_member_service(&self) -> Option<Arc<dyn WxMaLiveMemberService>> {
        Some(self.services().live_member.clone())
    }

    fn cloud_service(&self) -> Option<Arc<dyn WxMaCloudService>> {
        Some(self.services().cloud.clone())
    }

    fn vod_service(&self) -> Option<Arc<dyn WxMaVodService>> {
        Some(self.services().vod.clone())
    }

    fn xpay_service(&self) -> Option<Arc<dyn WxMaXPayService>> {
        Some(self.services().xpay.clone())
    }

    fn marketing_service(&self) -> Option<Arc<dyn WxMaMarketingService>> {
        Some(self.services().marketing.clone())
    }

    fn promotion_service(&self) -> Option<Arc<dyn WxMaPromotionService>> {
        Some(self.services().promotion.clone())
    }

    fn intracity_service(&self) -> Option<Arc<dyn WxMaIntracityService>> {
        Some(self.services().intracity.clone())
    }

    fn complaint_service(&self) -> Option<Arc<dyn WxMaComplaintService>> {
        Some(self.services().complaint.clone())
    }

    fn device_subscribe_service(&self) -> Option<Arc<dyn WxMaDeviceSubscribeService>> {
        Some(self.services().device_subscribe.clone())
    }

    fn face_service(&self) -> Option<Arc<dyn WxMaFaceService>> {
        Some(self.services().face.clone())
    }

    fn reimburse_invoice_service(&self) -> Option<Arc<dyn WxMaReimburseInvoiceService>> {
        Some(self.services().reimburse_invoice.clone())
    }

    fn qrcode_jump_service(&self) -> Option<Arc<dyn WxMaQrcodeJumpService>> {
        Some(self.services().qrcode_jump.clone())
    }

    fn ocr_service(&self) -> Option<Arc<dyn WxOcrService>> {
        Some(self.services().ocr.clone())
    }

    fn img_proc_service(&self) -> Option<Arc<dyn WxImgProcService>> {
        Some(self.services().img_proc.clone())
    }
}
