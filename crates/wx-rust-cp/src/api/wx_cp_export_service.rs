//! 异步导出服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpExportService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpExportRequest, WxCpExportResult};

/// 异步导出服务。
#[async_trait]
pub trait WxCpExportService: Send + Sync {
    /// 导出成员（对应 Java `WxCpExportService.simpleUser(WxCpExportRequest)`，
    /// 返回异步任务 id）。
    async fn simple_user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException>;

    /// 导出成员详情（对应 Java `WxCpExportService.user(WxCpExportRequest)`，
    /// 返回异步任务 id）。
    async fn user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException>;

    /// 导出部门（对应 Java
    /// `WxCpExportService.department(WxCpExportRequest)`，返回异步任务 id）。
    async fn department(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException>;

    /// 导出标签成员（对应 Java
    /// `WxCpExportService.tagUser(WxCpExportRequest)`，返回异步任务 id）。
    async fn tag_user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException>;

    /// 获取导出结果（对应 Java
    /// `WxCpExportService.getResult(String)`）。
    async fn get_result(&self, job_id: &str) -> Result<WxCpExportResult, WxErrorException>;
}
