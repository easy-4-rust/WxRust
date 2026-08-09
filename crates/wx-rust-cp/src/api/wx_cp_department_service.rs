//! 部门管理服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpDepartmentService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpDepart;

/// 部门管理服务。
#[async_trait]
pub trait WxCpDepartmentService: Send + Sync {
    /// 创建部门（对应 Java `WxCpDepartmentService.create(WxCpDepart)`，
    /// 返回新部门 id）。
    async fn create(&self, depart: &WxCpDepart) -> Result<i64, WxErrorException>;

    /// 获取部门详情（对应 Java `WxCpDepartmentService.get(Long)`）。
    async fn get(&self, id: i64) -> Result<WxCpDepart, WxErrorException>;

    /// 获取部门列表（对应 Java `WxCpDepartmentService.list(Long)`；
    /// `id` 非必填）。
    async fn list(&self, id: Option<i64>) -> Result<Vec<WxCpDepart>, WxErrorException>;

    /// 获取子部门 ID 列表（对应 Java
    /// `WxCpDepartmentService.simpleList(Long)`；`id` 非必填）。
    async fn simple_list(&self, id: Option<i64>) -> Result<Vec<WxCpDepart>, WxErrorException>;

    /// 更新部门（对应 Java `WxCpDepartmentService.update(WxCpDepart)`）。
    async fn update(&self, group: &WxCpDepart) -> Result<(), WxErrorException>;

    /// 删除部门（对应 Java `WxCpDepartmentService.delete(Long)`）。
    async fn delete(&self, depart_id: i64) -> Result<(), WxErrorException>;
}
