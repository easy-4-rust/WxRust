//! 企业微信日历服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaCalendarServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaCalendarService, WxCpService};
use crate::bean::WxCpOaCalendar;
use crate::enums::url_oa;

/// 企业微信日历服务实现。
pub struct WxCpOaCalendarServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaCalendarServiceImpl {
    /// 构建日历服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造获取日历请求体（对应 Java `get` 内的
    /// `GsonHelper.buildJsonObject("cal_id_list", calIds)`）。
    fn build_get_body(cal_ids: &[&str]) -> String {
        serde_json::json!({ "cal_id_list": cal_ids }).to_string()
    }

    /// 构造删除日历请求体（对应 Java `delete` 内的
    /// `GsonHelper.buildJsonObject("cal_id", calId)`）。
    fn build_delete_body(cal_id: &str) -> String {
        serde_json::json!({ "cal_id": cal_id }).to_string()
    }

    /// 从响应中解析 `calendar_list` 数组（对应 Java `get` 内
    /// `GsonParser.parse(response).get("calendar_list").getAsJsonArray()`
    /// + `TypeToken<List<WxCpOaCalendar>>`）。
    fn parse_calendar_list<T: serde::de::DeserializeOwned>(
        response: &str,
    ) -> Result<Vec<T>, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("calendar_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "calendar_list 字段缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpOaCalendarService for WxCpOaCalendarServiceImpl {
    async fn add(&self, calendar: &WxCpOaCalendar) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `add`：`POST CALENDAR_ADD`，直接返回响应内容（对应 Java
        // `this.wxCpService.post(...)` 的返回值）
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::CALENDAR_ADD);
        let body = calendar.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&api_url, &body).await
    }

    async fn update(&self, calendar: &WxCpOaCalendar) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：`POST CALENDAR_UPDATE`（更新操作是覆盖式）
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::CALENDAR_UPDATE);
        let body = calendar.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&api_url, &body).await?;
        Ok(())
    }

    async fn get(&self, cal_ids: &[&str]) -> Result<Vec<WxCpOaCalendar>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get`：`POST CALENDAR_GET`，解析 `calendar_list`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::CALENDAR_GET);
        let response = svc.post(&api_url, &Self::build_get_body(cal_ids)).await?;
        Self::parse_calendar_list(&response)
    }

    async fn delete(&self, cal_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete`：`POST CALENDAR_DEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::CALENDAR_DEL);
        svc.post(&api_url, &Self::build_delete_body(cal_id)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `get`：请求体 `{"cal_id_list":["calId1","calId2"]}`。
    #[test]
    fn test_build_get_body() {
        assert_eq!(
            WxCpOaCalendarServiceImpl::build_get_body(&["calId1", "calId2"]),
            r#"{"cal_id_list":["calId1","calId2"]}"#
        );
    }

    /// Java `delete`：请求体 `{"cal_id":"calId"}`。
    #[test]
    fn test_build_delete_body() {
        assert_eq!(
            WxCpOaCalendarServiceImpl::build_delete_body("calId"),
            r#"{"cal_id":"calId"}"#
        );
    }

    /// Java `get`：响应 `calendar_list` 解析为 `WxCpOaCalendar` 列表。
    #[test]
    fn test_parse_calendar_list() {
        let response = r#"{"errcode":0,"errmsg":"ok","calendar_list":[{"cal_id":"c1","summary":"日历一"},{"cal_id":"c2"}]}"#;
        let list = WxCpOaCalendarServiceImpl::parse_calendar_list::<WxCpOaCalendar>(response)
            .expect("解析失败");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].cal_id, "c1");
        assert_eq!(list[0].summary, "日历一");
        assert_eq!(list[1].cal_id, "c2");
    }
}
