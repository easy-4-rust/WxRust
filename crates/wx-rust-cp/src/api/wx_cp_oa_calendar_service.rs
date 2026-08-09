//! 企业微信日历服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaCalendarService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpOaCalendar;

/// 企业微信日历服务。
#[async_trait]
pub trait WxCpOaCalendarService: Send + Sync {
    /// 创建日历（对应 Java `WxCpOaCalendarService.add(WxCpOaCalendar)`，
    /// 返回日历 ID）。
    async fn add(&self, calendar: &WxCpOaCalendar) -> Result<String, WxErrorException>;

    /// 更新日历（对应 Java `WxCpOaCalendarService.update(WxCpOaCalendar)`；
    /// 更新操作是覆盖式，不是增量式）。
    async fn update(&self, calendar: &WxCpOaCalendar) -> Result<(), WxErrorException>;

    /// 获取日历（对应 Java
    /// `WxCpOaCalendarService.get(List<String>)`）。
    async fn get(&self, cal_ids: &[&str]) -> Result<Vec<WxCpOaCalendar>, WxErrorException>;

    /// 删除日历（对应 Java `WxCpOaCalendarService.delete(String)`）。
    async fn delete(&self, cal_id: &str) -> Result<(), WxErrorException>;
}
