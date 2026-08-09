//! 企业微信日程服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaScheduleService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpOaSchedule;

/// 企业微信日程服务。
#[async_trait]
pub trait WxCpOaScheduleService: Send + Sync {
    /// 创建日程（对应 Java
    /// `WxCpOaScheduleService.add(WxCpOaSchedule, Integer)`；`agentId`
    /// 仅旧的第三方多应用套件需要填）。
    async fn add(
        &self,
        schedule: &WxCpOaSchedule,
        agent_id: Option<i32>,
    ) -> Result<String, WxErrorException>;

    /// 更新日程（对应 Java
    /// `WxCpOaScheduleService.update(WxCpOaSchedule)`；更新操作是覆盖式）。
    async fn update(&self, schedule: &WxCpOaSchedule) -> Result<(), WxErrorException>;

    /// 获取日程详情（对应 Java
    /// `WxCpOaScheduleService.getDetails(List<String>)`）。
    async fn get_details(
        &self,
        schedule_ids: &[&str],
    ) -> Result<Vec<WxCpOaSchedule>, WxErrorException>;

    /// 取消日程（对应 Java `WxCpOaScheduleService.delete(String)`）。
    async fn delete(&self, schedule_id: &str) -> Result<(), WxErrorException>;

    /// 获取日历下的日程列表（对应 Java
    /// `WxCpOaScheduleService.listByCalendar(String, Integer, Integer)`）。
    async fn list_by_calendar(
        &self,
        cal_id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<WxCpOaSchedule>, WxErrorException>;
}
