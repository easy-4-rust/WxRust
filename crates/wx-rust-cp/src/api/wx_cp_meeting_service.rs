//! 企业微信会议服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpMeetingService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpMeeting, WxCpMeetingUpdateResult, WxCpUserMeetingIdResult};

/// 企业微信会议服务。
#[async_trait]
pub trait WxCpMeetingService: Send + Sync {
    /// 创建预约会议（对应 Java `WxCpMeetingService.create(WxCpMeeting)`，
    /// 返回会议 ID）。
    async fn create(&self, meeting: &WxCpMeeting) -> Result<String, WxErrorException>;

    /// 修改预约会议（对应 Java
    /// `WxCpMeetingService.update(WxCpMeeting)`）。
    async fn update(
        &self,
        meeting: &WxCpMeeting,
    ) -> Result<WxCpMeetingUpdateResult, WxErrorException>;

    /// 取消预约会议（对应 Java `WxCpMeetingService.cancel(String)`）。
    async fn cancel(&self, meeting_id: &str) -> Result<(), WxErrorException>;

    /// 获取会议详情（对应 Java `WxCpMeetingService.getDetail(String)`）。
    async fn get_detail(&self, meeting_id: &str) -> Result<WxCpMeeting, WxErrorException>;

    /// 获取成员会议 ID 列表（对应 Java
    /// `WxCpMeetingService.getUserMeetingIds(String, String, Integer, Long,
    /// Long)`；`beginTime`/`endTime` 为 Unix 时间戳）。
    async fn get_user_meeting_ids(
        &self,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
        begin_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<WxCpUserMeetingIdResult, WxErrorException>;
}
