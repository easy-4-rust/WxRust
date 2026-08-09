//! 菜单管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpMenuService`。

use async_trait::async_trait;

use wx_rust_common::bean::menu::WxMenu;
use wx_rust_common::error::WxErrorException;

/// 菜单管理服务。
#[async_trait]
pub trait WxCpMenuService: Send + Sync {
    /// 自定义菜单创建接口（对应 Java `WxCpMenuService.create(WxMenu)`；
    /// 使用 `WxCpConfigStorage` 里的 agentId）。
    async fn create(&self, menu: &WxMenu) -> Result<(), WxErrorException>;

    /// 自定义菜单创建接口（对应 Java
    /// `WxCpMenuService.create(Integer, WxMenu)`；不使用配置里的
    /// agentId，由调用方给出）。
    async fn create_with_agent_id(
        &self,
        agent_id: i32,
        menu: &WxMenu,
    ) -> Result<(), WxErrorException>;

    /// 自定义菜单删除接口（对应 Java `WxCpMenuService.delete()`；
    /// 使用 `WxCpConfigStorage` 里的 agentId）。
    async fn delete(&self) -> Result<(), WxErrorException>;

    /// 自定义菜单删除接口（对应 Java `WxCpMenuService.delete(Integer)`）。
    async fn delete_with_agent_id(&self, agent_id: i32) -> Result<(), WxErrorException>;

    /// 自定义菜单查询接口（对应 Java `WxCpMenuService.get()`；
    /// 使用 `WxCpConfigStorage` 里的 agentId；46003「不存在的菜单数据」
    /// 返回 `None`，对应 Java 返回 null）。
    async fn get(&self) -> Result<Option<WxMenu>, WxErrorException>;

    /// 自定义菜单查询接口（对应 Java `WxCpMenuService.get(Integer)`；
    /// 46003「不存在的菜单数据」返回 `None`，对应 Java 返回 null）。
    async fn get_with_agent_id(&self, agent_id: i32) -> Result<Option<WxMenu>, WxErrorException>;
}
