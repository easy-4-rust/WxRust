//! 菜单管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpMenuServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::bean::menu::WxMenu;
use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpMenuService, WxCpService};
use crate::enums::url_menu::*;

/// 菜单管理服务实现。
pub struct WxCpMenuServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpMenuServiceImpl {
    /// 构建菜单服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpMenuService for WxCpMenuServiceImpl {
    async fn create(&self, menu: &WxMenu) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create(WxMenu)`：使用配置里的 agentId
        let config = svc.wx_cp_config_storage();
        let agent_id = config.agent_id().unwrap_or(0);
        self.create_with_agent_id(agent_id, menu).await
    }

    async fn create_with_agent_id(
        &self,
        agent_id: i32,
        menu: &WxMenu,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create(Integer, WxMenu)`：GET `String.format(MENU_CREATE,
        // agentId)`（`%d` 替换）后 POST `menu.toJson()`
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(MENU_CREATE)
            .replace("%d", &agent_id.to_string());
        svc.post(&url, &menu.to_json()).await?;
        Ok(())
    }

    async fn delete(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete()`：使用配置里的 agentId
        let config = svc.wx_cp_config_storage();
        let agent_id = config.agent_id().unwrap_or(0);
        self.delete_with_agent_id(agent_id).await
    }

    async fn delete_with_agent_id(&self, agent_id: i32) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete(Integer)`：GET `String.format(MENU_DELETE, agentId)`
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(MENU_DELETE)
            .replace("%d", &agent_id.to_string());
        svc.get(&url, "").await?;
        Ok(())
    }

    async fn get(&self) -> Result<Option<WxMenu>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get()`：使用配置里的 agentId
        let config = svc.wx_cp_config_storage();
        let agent_id = config.agent_id().unwrap_or(0);
        self.get_with_agent_id(agent_id).await
    }

    async fn get_with_agent_id(&self, agent_id: i32) -> Result<Option<WxMenu>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get(Integer)`：GET `String.format(MENU_GET, agentId)`；
        // 46003「不存在的菜单数据」返回 null（Rust 以 None 表达，ADAPTED）
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(MENU_GET)
            .replace("%d", &agent_id.to_string());
        match svc.get(&url, "").await {
            Ok(result_content) => WxMenu::from_json(&result_content)
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            Err(e) => {
                if e.error_code() == Some(46003) {
                    return Ok(None);
                }
                Err(e)
            }
        }
    }
}
