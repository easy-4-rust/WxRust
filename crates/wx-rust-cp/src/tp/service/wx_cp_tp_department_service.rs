//! 企业微信第三方应用部门服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpDepartmentService`：
//! 部门管理接口（创建/列表/更新/删除，最多支持创建 500 个部门）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpTpDepart;

/// 企业微信第三方应用部门服务。
#[async_trait]
pub trait WxCpTpDepartmentService: Send + Sync {
    /// 创建部门（对应 Java `create(WxCpTpDepart)`，返回部门 id）。
    async fn create(&self, depart: &WxCpTpDepart) -> Result<i64, WxErrorException>;

    /// 获取部门列表（对应 Java `list(Long, String)`：id 为 null 时获取
    /// 全部部门）。
    async fn list(
        &self,
        id: Option<i64>,
        corp_id: &str,
    ) -> Result<Vec<WxCpTpDepart>, WxErrorException>;

    /// 更新部门（对应 Java `update(WxCpTpDepart)`：id/name 必须设置）。
    async fn update(&self, group: &WxCpTpDepart) -> Result<(), WxErrorException>;

    /// 删除部门（对应 Java `delete(Long)`）。
    async fn delete(&self, depart_id: i64) -> Result<(), WxErrorException>;

    /// 获取所有部门列表（对应 Java `list(String)` 单参重载）。
    async fn list_all(&self, corp_id: &str) -> Result<Vec<WxCpTpDepart>, WxErrorException> {
        self.list(None, corp_id).await
    }
}
