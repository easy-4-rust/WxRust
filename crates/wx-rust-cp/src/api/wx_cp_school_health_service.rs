//! 企业微信家校应用健康上报服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpSchoolHealthService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpGetHealthReportStat, WxCpGetReportAnswer, WxCpGetReportJobIds, WxCpGetReportJobInfo,
};

/// 企业微信家校应用健康上报服务。
#[async_trait]
pub trait WxCpSchoolHealthService: Send + Sync {
    /// 获取健康上报使用统计（对应 Java
    /// `WxCpSchoolHealthService.getHealthReportStat(String)`；`date` 最长
    /// 支持获取 30 天前数据）。
    async fn get_health_report_stat(
        &self,
        date: &str,
    ) -> Result<WxCpGetHealthReportStat, WxErrorException>;

    /// 获取健康上报任务 ID 列表（对应 Java
    /// `WxCpSchoolHealthService.getReportJobIds(Integer, Integer)`）。
    async fn get_report_job_ids(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpGetReportJobIds, WxErrorException>;

    /// 获取健康上报任务详情（对应 Java
    /// `WxCpSchoolHealthService.getReportJobInfo(String, String)`；
    /// `date` 仅支持最近 14 天数据）。
    async fn get_report_job_info(
        &self,
        job_id: &str,
        date: &str,
    ) -> Result<WxCpGetReportJobInfo, WxErrorException>;

    /// 获取用户填写答案（对应 Java
    /// `WxCpSchoolHealthService.getReportAnswer(String, String, Integer,
    /// Integer)`）。
    async fn get_report_answer(
        &self,
        job_id: &str,
        date: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpGetReportAnswer, WxErrorException>;
}
