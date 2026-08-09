//! 小程序直播成员管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaLiveMemberService`
//! （`impl.WxMaLiveMemberServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

/// 小程序直播成员管理服务。
#[async_trait]
pub trait WxMaLiveMemberService: Send + Sync {
    /// 设置成员角色（对应 Java `addRole`，返回微信原始响应报文）。
    async fn add_role(&self, username: &str, role: i32) -> Result<String, WxErrorException>;

    /// 解除成员角色（对应 Java `deleteRole`，返回微信原始响应报文）。
    async fn delete_role(&self, username: &str, role: i32) -> Result<String, WxErrorException>;

    /// 查询成员列表（对应 Java `listByRole`，返回 `list` 节点 JSON 数组）。
    async fn list_by_role(
        &self,
        role: i32,
        offset: i32,
        limit: i32,
        keyword: Option<&str>,
    ) -> Result<serde_json::Value, WxErrorException>;
}
