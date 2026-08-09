//! 微信营销服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaMarketingService`
//! （`impl.WxMaMarketingServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::marketing::WxMaUserAction;

/// 微信营销服务。
///
/// 对应 Java `WxMaMarketingService`：创建数据源（用户行为源）与回传数据。
#[async_trait]
pub trait WxMaMarketingService: Send + Sync {
    /// 创建数据源（对应 Java `addUserActionSets`，返回 `data.user_action_set_id`）。
    async fn add_user_action_sets(
        &self,
        r#type: &str,
        name: &str,
        description: &str,
    ) -> Result<i64, WxErrorException>;

    /// 回传数据（对应 Java `addUserAction`，返回微信原始响应报文）。
    async fn add_user_action(
        &self,
        actions: &[WxMaUserAction],
        user_action_set_id: Option<i64>,
    ) -> Result<String, WxErrorException>;
}
