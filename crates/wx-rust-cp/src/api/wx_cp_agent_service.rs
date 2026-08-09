//! 企业号应用管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpAgentService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpAgent, WxCpTpAdmin};

/// 企业号应用管理服务。
#[async_trait]
pub trait WxCpAgentService: Send + Sync {
    /// 获取企业号应用信息（对应 Java `WxCpAgentService.get(Integer)`）。
    async fn get(&self, agent_id: i32) -> Result<WxCpAgent, WxErrorException>;

    /// 设置应用（对应 Java `WxCpAgentService.set(WxCpAgent)`）。
    async fn set(&self, agent_info: &WxCpAgent) -> Result<(), WxErrorException>;

    /// 获取应用列表（对应 Java `WxCpAgentService.list()`）。
    async fn list(&self) -> Result<Vec<WxCpAgent>, WxErrorException>;

    /// 获取应用管理员列表（对应 Java
    /// `WxCpAgentService.getAdminList(Integer)`）。
    async fn get_admin_list(&self, agent_id: i32) -> Result<WxCpTpAdmin, WxErrorException>;
}
