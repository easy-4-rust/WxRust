//! 公众号服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpServiceImpl`：组合
//! 门面 trait 的默认实现 + 多公众号配置管理（对应 Java `configStorageMap` +
//! `WxMpConfigStorageHolder`）。子服务以 `Weak<dyn WxMpService>` 注入
//! （对应 Java `new WxMpMenuServiceImpl(this)` 的循环引用，Rust 用弱引用打破）。

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use crate::api::r#impl::{
    WxMpAiOpenServiceImpl, WxMpCardServiceImpl, WxMpCommentServiceImpl, WxMpDataCubeServiceImpl,
    WxMpDeviceServiceImpl, WxMpDraftServiceImpl, WxMpFreePublishServiceImpl,
    WxMpGuideBuyerServiceImpl, WxMpGuideMassedJobServiceImpl, WxMpGuideMaterialServiceImpl,
    WxMpGuideServiceImpl, WxMpGuideTagServiceImpl, WxMpImgProcServiceImpl, WxMpKefuServiceImpl,
    WxMpMarketingServiceImpl, WxMpMassMessageServiceImpl, WxMpMaterialServiceImpl,
    WxMpMemberCardServiceImpl, WxMpMenuServiceImpl, WxMpMerchantInvoiceServiceImpl,
    WxMpOcrServiceImpl, WxMpQrcodeServiceImpl, WxMpReimburseInvoiceServiceImpl,
    WxMpShakeServiceImpl, WxMpStoreServiceImpl, WxMpSubscribeMsgServiceImpl,
    WxMpTemplateMsgServiceImpl, WxMpUserBlacklistServiceImpl, WxMpUserServiceImpl,
    WxMpUserTagServiceImpl, WxMpWifiServiceImpl,
};
use crate::api::{
    WxMpAiOpenService, WxMpCardService, WxMpCommentService, WxMpDataCubeService, WxMpDeviceService,
    WxMpDraftService, WxMpFreePublishService, WxMpGuideBuyerService, WxMpGuideMassedJobService,
    WxMpGuideMaterialService, WxMpGuideService, WxMpGuideTagService, WxMpImgProcService,
    WxMpKefuService, WxMpMarketingService, WxMpMassMessageService, WxMpMaterialService,
    WxMpMemberCardService, WxMpMenuService, WxMpMerchantInvoiceService, WxMpOcrService,
    WxMpQrcodeService, WxMpReimburseInvoiceService, WxMpService, WxMpShakeService,
    WxMpStoreService, WxMpSubscribeMsgService, WxMpTemplateMsgService, WxMpUserBlacklistService,
    WxMpUserService, WxMpUserTagService, WxMpWifiService,
};
use crate::config::WxMpConfigStorage;
use crate::util::wx_mp_config_storage_holder;

/// 子服务集合（对应 Java Base 构造器中一次性实例化的子服务字段）。
struct SubServices {
    menu: Arc<dyn WxMpMenuService>,
    template_msg: Arc<dyn WxMpTemplateMsgService>,
    qrcode: Arc<dyn WxMpQrcodeService>,
    kefu: Arc<dyn WxMpKefuService>,
    user: Arc<dyn WxMpUserService>,
    user_tag: Arc<dyn WxMpUserTagService>,
    user_blacklist: Arc<dyn WxMpUserBlacklistService>,
    store: Arc<dyn WxMpStoreService>,
    comment: Arc<dyn WxMpCommentService>,
    data_cube: Arc<dyn WxMpDataCubeService>,
    wifi: Arc<dyn WxMpWifiService>,
    draft: Arc<dyn WxMpDraftService>,
    free_publish: Arc<dyn WxMpFreePublishService>,
    device: Arc<dyn WxMpDeviceService>,
    mass_message: Arc<dyn WxMpMassMessageService>,
    material: Arc<dyn WxMpMaterialService>,
    shake: Arc<dyn WxMpShakeService>,
    card: Arc<dyn WxMpCardService>,
    member_card: Arc<dyn WxMpMemberCardService>,
    guide: Arc<dyn WxMpGuideService>,
    marketing: Arc<dyn WxMpMarketingService>,
    subscribe_msg: Arc<dyn WxMpSubscribeMsgService>,
    ai_open: Arc<dyn WxMpAiOpenService>,
    ocr: Arc<dyn WxMpOcrService>,
    img_proc: Arc<dyn WxMpImgProcService>,
    reimburse_invoice: Arc<dyn WxMpReimburseInvoiceService>,
    merchant_invoice: Arc<dyn WxMpMerchantInvoiceService>,
    guide_buyer: Arc<dyn WxMpGuideBuyerService>,
    guide_tag: Arc<dyn WxMpGuideTagService>,
    guide_material: Arc<dyn WxMpGuideMaterialService>,
    guide_massed_job: Arc<dyn WxMpGuideMassedJobService>,
}

/// 公众号服务实现（reqwest HTTP 后端）。
pub struct WxMpServiceImpl {
    client: reqwest::Client,
    config_storages: Mutex<HashMap<String, Arc<dyn WxMpConfigStorage>>>,
    default_appid: String,
    sub_services: OnceLock<SubServices>,
}

impl WxMpServiceImpl {
    /// 构建服务（子服务注入 `Weak<dyn WxMpService>` 打破循环引用）。
    ///
    /// # 参数
    /// - `config`：初始公众号配置
    pub fn new_arc(config: Arc<dyn WxMpConfigStorage>) -> Arc<Self> {
        let appid = config.app_id().to_string();
        let mut storages = HashMap::new();
        storages.insert(appid.clone(), config);
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storages: Mutex::new(storages),
            default_appid: appid,
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxMpService> 再降级为 Weak<dyn WxMpService>
        let dyn_arc: Arc<dyn WxMpService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            menu: Arc::new(WxMpMenuServiceImpl::new(weak.clone())),
            template_msg: Arc::new(WxMpTemplateMsgServiceImpl::new(weak.clone())),
            qrcode: Arc::new(WxMpQrcodeServiceImpl::new(weak.clone())),
            kefu: Arc::new(WxMpKefuServiceImpl::new(weak.clone())),
            user: Arc::new(WxMpUserServiceImpl::new(weak.clone())),
            user_tag: Arc::new(WxMpUserTagServiceImpl::new(weak.clone())),
            user_blacklist: Arc::new(WxMpUserBlacklistServiceImpl::new(weak.clone())),
            store: Arc::new(WxMpStoreServiceImpl::new(weak.clone())),
            comment: Arc::new(WxMpCommentServiceImpl::new(weak.clone())),
            data_cube: Arc::new(WxMpDataCubeServiceImpl::new(weak.clone())),
            wifi: Arc::new(WxMpWifiServiceImpl::new(weak.clone())),
            draft: Arc::new(WxMpDraftServiceImpl::new(weak.clone())),
            free_publish: Arc::new(WxMpFreePublishServiceImpl::new(weak.clone())),
            device: Arc::new(WxMpDeviceServiceImpl::new(weak.clone())),
            mass_message: Arc::new(WxMpMassMessageServiceImpl::new(weak.clone())),
            material: Arc::new(WxMpMaterialServiceImpl::new(weak.clone())),
            shake: Arc::new(WxMpShakeServiceImpl::new(weak.clone())),
            card: Arc::new(WxMpCardServiceImpl::new(weak.clone())),
            member_card: Arc::new(WxMpMemberCardServiceImpl::new(weak.clone())),
            guide: Arc::new(WxMpGuideServiceImpl::new(weak.clone())),
            marketing: Arc::new(WxMpMarketingServiceImpl::new(weak.clone())),
            subscribe_msg: Arc::new(WxMpSubscribeMsgServiceImpl::new(weak.clone())),
            ai_open: Arc::new(WxMpAiOpenServiceImpl::new(weak.clone())),
            ocr: Arc::new(WxMpOcrServiceImpl::new(weak.clone())),
            img_proc: Arc::new(WxMpImgProcServiceImpl::new(weak.clone())),
            reimburse_invoice: Arc::new(WxMpReimburseInvoiceServiceImpl::new(weak.clone())),
            merchant_invoice: Arc::new(WxMpMerchantInvoiceServiceImpl::new(weak.clone())),
            guide_buyer: Arc::new(WxMpGuideBuyerServiceImpl::new(weak.clone())),
            guide_tag: Arc::new(WxMpGuideTagServiceImpl::new(weak.clone())),
            guide_material: Arc::new(WxMpGuideMaterialServiceImpl::new(weak.clone())),
            guide_massed_job: Arc::new(WxMpGuideMassedJobServiceImpl::new(weak.clone())),
        });
        arc
    }

    /// 子服务集合。
    fn services(&self) -> &SubServices {
        self.sub_services.get().expect("子服务已在构建时安装")
    }

    /// 设置初始公众号配置（对应 Java `setWxMpConfigStorage`：appid 不能为 null）。
    pub fn set_config_storage(&self, config: Arc<dyn WxMpConfigStorage>) -> Result<(), String> {
        let appid = config.app_id();
        if appid.is_empty() {
            return Err("appid不能设置为null".to_string());
        }
        let mut map = self.config_storages.lock().unwrap();
        map.clear();
        map.insert(appid.to_string(), config);
        Ok(())
    }

    /// 添加多公众号配置（对应 Java `addConfigStorage`）。
    pub fn add_config_storage(&self, mp_id: &str, config: Arc<dyn WxMpConfigStorage>) {
        let mut map = self.config_storages.lock().unwrap();
        map.insert(mp_id.to_string(), config);
        wx_mp_config_storage_holder::set(mp_id);
    }

    /// 移除公众号配置（对应 Java `removeConfigStorage`）。
    pub fn remove_config_storage(&self, mp_id: &str) {
        let mut map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            map.remove(mp_id);
            return;
        }
        map.remove(mp_id);
        if let Some(next) = map.keys().next() {
            wx_mp_config_storage_holder::set(next.clone());
        }
    }

    /// 切换到指定公众号（对应 Java `switchoverTo`）。
    pub fn switchover_to(&self, mp_id: &str) -> Result<(), String> {
        let map = self.config_storages.lock().unwrap();
        if map.contains_key(mp_id) {
            wx_mp_config_storage_holder::set(mp_id);
            Ok(())
        } else {
            Err(format!("无法找到对应【{mp_id}】的公众号配置信息，请核实！"))
        }
    }
}

impl WxMpService for WxMpServiceImpl {
    fn wx_mp_config_storage(&self) -> Arc<dyn WxMpConfigStorage> {
        let map = self.config_storages.lock().unwrap();
        if map.len() == 1 {
            // 只有一个公众号，直接返回其配置
            return map.values().next().unwrap().clone();
        }
        let holder = wx_mp_config_storage_holder::get();
        map.get(&holder)
            .cloned()
            .or_else(|| map.get(&self.default_appid).cloned())
            .unwrap_or_else(|| map.values().next().unwrap().clone())
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn menu_service(&self) -> Option<Arc<dyn WxMpMenuService>> {
        Some(self.services().menu.clone())
    }

    fn template_msg_service(&self) -> Option<Arc<dyn WxMpTemplateMsgService>> {
        Some(self.services().template_msg.clone())
    }

    fn qrcode_service(&self) -> Option<Arc<dyn WxMpQrcodeService>> {
        Some(self.services().qrcode.clone())
    }

    fn kefu_service(&self) -> Option<Arc<dyn WxMpKefuService>> {
        Some(self.services().kefu.clone())
    }

    fn user_service(&self) -> Option<Arc<dyn WxMpUserService>> {
        Some(self.services().user.clone())
    }

    fn user_tag_service(&self) -> Option<Arc<dyn WxMpUserTagService>> {
        Some(self.services().user_tag.clone())
    }

    fn user_blacklist_service(&self) -> Option<Arc<dyn WxMpUserBlacklistService>> {
        Some(self.services().user_blacklist.clone())
    }

    fn store_service(&self) -> Option<Arc<dyn WxMpStoreService>> {
        Some(self.services().store.clone())
    }

    fn comment_service(&self) -> Option<Arc<dyn WxMpCommentService>> {
        Some(self.services().comment.clone())
    }

    fn data_cube_service(&self) -> Option<Arc<dyn WxMpDataCubeService>> {
        Some(self.services().data_cube.clone())
    }

    fn wifi_service(&self) -> Option<Arc<dyn WxMpWifiService>> {
        Some(self.services().wifi.clone())
    }

    fn draft_service(&self) -> Option<Arc<dyn WxMpDraftService>> {
        Some(self.services().draft.clone())
    }

    fn free_publish_service(&self) -> Option<Arc<dyn WxMpFreePublishService>> {
        Some(self.services().free_publish.clone())
    }

    fn device_service(&self) -> Option<Arc<dyn WxMpDeviceService>> {
        Some(self.services().device.clone())
    }

    fn mass_message_service(&self) -> Option<Arc<dyn WxMpMassMessageService>> {
        Some(self.services().mass_message.clone())
    }

    fn guide_buyer_service(&self) -> Option<Arc<dyn WxMpGuideBuyerService>> {
        Some(self.services().guide_buyer.clone())
    }

    fn guide_tag_service(&self) -> Option<Arc<dyn WxMpGuideTagService>> {
        Some(self.services().guide_tag.clone())
    }

    fn guide_material_service(&self) -> Option<Arc<dyn WxMpGuideMaterialService>> {
        Some(self.services().guide_material.clone())
    }

    fn guide_massed_job_service(&self) -> Option<Arc<dyn WxMpGuideMassedJobService>> {
        Some(self.services().guide_massed_job.clone())
    }

    fn material_service(&self) -> Option<Arc<dyn WxMpMaterialService>> {
        Some(self.services().material.clone())
    }

    fn shake_service(&self) -> Option<Arc<dyn WxMpShakeService>> {
        Some(self.services().shake.clone())
    }

    fn card_service(&self) -> Option<Arc<dyn WxMpCardService>> {
        Some(self.services().card.clone())
    }

    fn member_card_service(&self) -> Option<Arc<dyn WxMpMemberCardService>> {
        Some(self.services().member_card.clone())
    }

    fn guide_service(&self) -> Option<Arc<dyn WxMpGuideService>> {
        Some(self.services().guide.clone())
    }

    fn marketing_service(&self) -> Option<Arc<dyn WxMpMarketingService>> {
        Some(self.services().marketing.clone())
    }

    fn subscribe_msg_service(&self) -> Option<Arc<dyn WxMpSubscribeMsgService>> {
        Some(self.services().subscribe_msg.clone())
    }

    fn ai_open_service(&self) -> Option<Arc<dyn WxMpAiOpenService>> {
        Some(self.services().ai_open.clone())
    }

    fn ocr_service(&self) -> Option<Arc<dyn WxMpOcrService>> {
        Some(self.services().ocr.clone())
    }

    fn img_proc_service(&self) -> Option<Arc<dyn WxMpImgProcService>> {
        Some(self.services().img_proc.clone())
    }

    fn reimburse_invoice_service(&self) -> Option<Arc<dyn WxMpReimburseInvoiceService>> {
        Some(self.services().reimburse_invoice.clone())
    }

    fn merchant_invoice_service(&self) -> Option<Arc<dyn WxMpMerchantInvoiceService>> {
        Some(self.services().merchant_invoice.clone())
    }
}
