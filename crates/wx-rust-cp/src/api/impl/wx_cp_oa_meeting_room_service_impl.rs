//! 企业微信会议室服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaMeetingRoomServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaMeetingRoomService, WxCpService};
use crate::bean::{
    WxCpOaMeetingRoom, WxCpOaMeetingRoomBookByMeetingRequest,
    WxCpOaMeetingRoomBookByScheduleRequest, WxCpOaMeetingRoomBookRequest,
    WxCpOaMeetingRoomBookResult, WxCpOaMeetingRoomBookingInfoByBookingIdRequest,
    WxCpOaMeetingRoomBookingInfoByBookingIdResult, WxCpOaMeetingRoomBookingInfoRequest,
    WxCpOaMeetingRoomBookingInfoResult, WxCpOaMeetingRoomCancelBookRequest,
};
use crate::enums::url_oa;

/// 企业微信会议室服务实现。
pub struct WxCpOaMeetingRoomServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaMeetingRoomServiceImpl {
    /// 构建会议室服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造删除会议室请求体（对应 Java `deleteMeetingRoom` 内的
    /// `GsonHelper.buildJsonObject("meetingroom_id", meetingRoomId)`）。
    fn build_delete_body(meeting_room_id: i32) -> String {
        serde_json::json!({ "meetingroom_id": meeting_room_id }).to_string()
    }

    /// 从响应中解析 `meetingroom_list` 数组（对应 Java `listMeetingRoom`
    /// 内 `GsonParser.parse(response).get("meetingroom_list").getAsJsonArray()`
    /// + `TypeToken<List<WxCpOaMeetingRoom>>`）。
    fn parse_meeting_room_list<T: serde::de::DeserializeOwned>(
        response: &str,
    ) -> Result<Vec<T>, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("meetingroom_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "meetingroom_list 字段缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpOaMeetingRoomService for WxCpOaMeetingRoomServiceImpl {
    async fn add_meeting_room(
        &self,
        meeting_room: &WxCpOaMeetingRoom,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `addMeetingRoom`：`POST MEETINGROOM_ADD`，直接返回响应内容
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::MEETINGROOM_ADD);
        let body = meeting_room.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&api_url, &body).await
    }

    async fn list_meeting_room(
        &self,
        meeting_room_request: &WxCpOaMeetingRoom,
    ) -> Result<Vec<WxCpOaMeetingRoom>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listMeetingRoom`：`POST MEETINGROOM_LIST`，解析
        // `meetingroom_list`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::MEETINGROOM_LIST);
        let body = meeting_room_request
            .to_json()
            .map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        Self::parse_meeting_room_list(&response)
    }

    async fn edit_meeting_room(
        &self,
        meeting_room: &WxCpOaMeetingRoom,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `editMeetingRoom`：`POST MEETINGROOM_EDIT`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::MEETINGROOM_EDIT);
        let body = meeting_room.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&api_url, &body).await?;
        Ok(())
    }

    async fn delete_meeting_room(&self, meeting_room_id: i32) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteMeetingRoom`：`POST MEETINGROOM_DEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::MEETINGROOM_DEL);
        svc.post(&api_url, &Self::build_delete_body(meeting_room_id))
            .await?;
        Ok(())
    }

    async fn get_meeting_room_booking_info(
        &self,
        request: &WxCpOaMeetingRoomBookingInfoRequest,
    ) -> Result<WxCpOaMeetingRoomBookingInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getMeetingRoomBookingInfo`：`POST MEETINGROOM_GET_BOOKING_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::MEETINGROOM_GET_BOOKING_INFO);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpOaMeetingRoomBookingInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn booking_meeting_room(
        &self,
        request: &WxCpOaMeetingRoomBookRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `bookingMeetingRoom`：`POST MEETINGROOM_BOOK`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::MEETINGROOM_BOOK);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpOaMeetingRoomBookResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn booking_meeting_room_by_schedule(
        &self,
        request: &WxCpOaMeetingRoomBookByScheduleRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `bookingMeetingRoomBySchedule`：`POST MEETINGROOM_BOOK_BY_SCHEDULE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::MEETINGROOM_BOOK_BY_SCHEDULE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpOaMeetingRoomBookResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn booking_meeting_room_by_meeting(
        &self,
        request: &WxCpOaMeetingRoomBookByMeetingRequest,
    ) -> Result<WxCpOaMeetingRoomBookResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `bookingMeetingRoomByMeeting`：`POST MEETINGROOM_BOOK_BY_MEETING`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::MEETINGROOM_BOOK_BY_MEETING);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpOaMeetingRoomBookResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn cancel_book_meeting_room(
        &self,
        request: &WxCpOaMeetingRoomCancelBookRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `cancelBookMeetingRoom`：`POST MEETINGROOM_CANCEL_BOOK`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::MEETINGROOM_CANCEL_BOOK);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&api_url, &body).await?;
        Ok(())
    }

    async fn get_booking_info_by_booking_id(
        &self,
        request: &WxCpOaMeetingRoomBookingInfoByBookingIdRequest,
    ) -> Result<WxCpOaMeetingRoomBookingInfoByBookingIdResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getBookingInfoByBookingId`：`POST MEETINGROOM_BOOKINFO_GET`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::MEETINGROOM_BOOKINFO_GET);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpOaMeetingRoomBookingInfoByBookingIdResult::from_json(&response)
            .map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `deleteMeetingRoom`：请求体 `{"meetingroom_id":1001}`。
    #[test]
    fn test_build_delete_body() {
        assert_eq!(
            WxCpOaMeetingRoomServiceImpl::build_delete_body(1001),
            r#"{"meetingroom_id":1001}"#
        );
    }

    /// Java `listMeetingRoom`：响应 `meetingroom_list` 解析为列表。
    #[test]
    fn test_parse_meeting_room_list() {
        let response = r#"{"errcode":0,"errmsg":"ok","meetingroom_list":[{"meetingroom_id":1001,"name":"会议室A"},{"meetingroom_id":1002,"name":"会议室B"}]}"#;
        let list =
            WxCpOaMeetingRoomServiceImpl::parse_meeting_room_list::<WxCpOaMeetingRoom>(response)
                .expect("解析失败");
        assert_eq!(list.len(), 2);
        assert_eq!(list[1].name, "会议室B");
    }
}
