//! 菜单服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpMenuServiceImpl`。

use async_trait::async_trait;
use std::sync::{Arc, Weak};

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpMenuService, WxMpService};
use crate::bean::menu::WxMpMenu;
use crate::enums::wx_mp_api_url::menu as menu_url;

/// 菜单服务实现。
pub struct WxMpMenuServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpMenuServiceImpl {
    /// 构建菜单服务。
    ///
    /// # 参数
    /// - `service`：公众号服务弱引用（`Arc::new_cyclic` 注入）
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpMenuService for WxMpMenuServiceImpl {
    async fn menu_create(&self, menu: &WxMpMenu) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = menu_url::menu_create(config.as_ref());
        let body =
            serde_json::to_string(menu).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&url, &body).await
    }

    async fn menu_get(&self) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = menu_url::menu_get(config.as_ref());
        svc.get(&url, "").await
    }

    async fn menu_delete(&self) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = menu_url::menu_delete(config.as_ref());
        svc.get(&url, "").await
    }

    async fn get_self_menu_info(&self) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let url = menu_url::get_self_menu_info(config.as_ref());
        svc.get(&url, "").await
    }
}

/// 供门面持有时使用的 `Arc` 包装（与 Java `new WxMpMenuServiceImpl(this)` 对应）。
pub type MenuServiceArc = Arc<dyn WxMpMenuService>;
