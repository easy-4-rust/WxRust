//! 公众号菜单服务。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMenuService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::menu::WxMpMenu;

/// 菜单服务。
#[async_trait]
pub trait WxMpMenuService: Send + Sync {
    /// 创建菜单。
    ///
    /// # 参数
    /// - `menu`：菜单对象
    ///
    /// # 返回
    /// 接口响应（`{"errcode":0,...}` 的原始 JSON）。
    async fn menu_create(&self, menu: &WxMpMenu) -> Result<String, WxErrorException>;

    /// 获取菜单。
    async fn menu_get(&self) -> Result<String, WxErrorException>;

    /// 删除菜单。
    async fn menu_delete(&self) -> Result<String, WxErrorException>;

    /// 获取当前自定义菜单配置。
    async fn get_self_menu_info(&self) -> Result<String, WxErrorException>;
}

/// 提供方引用（实现内部使用，返回 `Arc<dyn WxMpService>` 的弱引用升级结果）。
pub type ServiceRef = std::sync::Weak<dyn crate::api::WxMpService>;
