//! 企业微信会议室服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaMeetingRoomService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpOaMeetingRoom, WxCpOaMeetingRoomBookByMeetingRequest,
    WxCpOaMeetingRoomBookByScheduleRequest, WxCpOaMeetingRoomBookRequest,
    WxCpOaMeetingRoomBookResult, WxCpOaMeetingRoomBookingInfoByBookingIdRequest,
    WxCpOaMeetingRoomBookingInfoByBookingIdResult, WxCpOaMeetingRoomBookingInfoRequest,
    WxCpOaMeetingRoomBookingInfoResult, WxCpOaMeetingRoomCancelBookRequest,
};

/// 企业微信会议室服务。
#[async_trait]
pub trait WxCpOaMeetingRoomService: Send + Sync {
    /// 创建会议室（对应 Java
    /// `WxCpOaMeetingRoomService.addMeetingRoom(WxCpOaMeetingRoom)`，
    /// 返回会议室 ID）。
    async fn add_meeting_room(
        &self,
        meeting_room: &WxCpOaMeetingRoom,
    ) -> Result<String, WxErrorException>;

    /// 查询会议室（对应 Java
    /// `WxCpOaMeetingRoomService.listMeetingRoom(WxCpOaMeetingRoom)`）。
    async fn list_meeting_room(
        &self,
        meeting_room_request: &WxCpOaMeetingRoom,
    ) -> Result<Vec<WxCpOaMeetingRoom>, WxErrorException>;

    /// 编辑会议室（对应 Java
    /// `WxCpOaMeetingRoomService.editMeetingRoom(WxCpOaMeetingRoom)`）。
    async fn edit_meeting_room(
        &self,
        meeting_room: &WxCpOaMeetingRoom,
    ) -> Result<(), WxErrorException>;

    /// 删除会议室（对应 Java
    /// `WxCpOaMeetingRoomService.deleteMeetingRoom(Integer)`）。
    async fn delete_meeting_room(&self, meeting_room_id: i32) -> Result<(), WxErrorException>;

    /// 查询会议室的预定信息（对应 Java
    /// `WxCpOaMeetingRoomService.getMeetingRoomBookingInfo(WxCpOaMeetingRoomBookingInfoRequest)`）。
    async fn get_meeting_room_booking_info(
        &self,
        request: &WxCpOaMeetingRoomBookingInfoRequest,
    ) -> Result<WxCpOaMeetingRoomBookingInfoResult, WxErrorException>;

    /// 预定会议室（对应 Java
    /// `WxCpOaMeetingRoomService.bookingMeetingRoom(WxCpOaMeetingRoomBookRequest)`）。
    async fn booking_meeting_room(
        &self,
        request: &WxCpOaMeetingRoomBookRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException>;

    /// 通过日程预定会议室（对应 Java
    /// `WxCpOaMeetingRoomService.bookingMeetingRoomBySchedule(WxCpOaMeetingRoomBookByScheduleRequest)`）。
    async fn booking_meeting_room_by_schedule(
        &self,
        request: &WxCpOaMeetingRoomBookByScheduleRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException>;

    /// 通过会议预定会议室（对应 Java
    /// `WxCpOaMeetingRoomService.bookingMeetingRoomByMeeting(WxCpOaMeetingRoomBookByMeetingRequest)`）。
    async fn booking_meeting_room_by_meeting(
        &self,
        request: &WxCpOaMeetingRoomBookByMeetingRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException>;

    /// 取消预定会议室（对应 Java
    /// `WxCpOaMeetingRoomService.cancelBookMeetingRoom(WxCpOaMeetingRoomCancelBookRequest)`）。
    async fn cancel_book_meeting_room(
        &self,
        request: &WxCpOaMeetingRoomCancelBookRequest,
    ) -> Result<(), WxErrorException>;

    /// 根据会议室预定 ID 查询预定详情（对应 Java
    /// `WxCpOaMeetingRoomService.getBookingInfoByBookingId(WxCpOaMeetingRoomBookingInfoByBookingIdRequest)`）。
    async fn get_booking_info_by_booking_id(
        &self,
        request: &WxCpOaMeetingRoomBookingInfoByBookingIdRequest,
    ) -> Result<WxCpOaMeetingRoomBookingInfoByBookingIdResult, WxErrorException>;
}
