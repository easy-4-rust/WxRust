//! 企业微信日程服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaOaScheduleServiceImpl`
//! （Java 类名为 `WxCpOaOaScheduleServiceImpl`，实现
//! `WxCpOaScheduleService`）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaScheduleService, WxCpService};
use crate::bean::WxCpOaSchedule;
use crate::enums::url_oa;

/// 企业微信日程服务实现。
pub struct WxCpOaScheduleServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaScheduleServiceImpl {
    /// 构建日程服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造添加日程请求体（对应 Java `add` 内的
    /// `ImmutableMap.of("schedule", schedule)`，`agentId` 非空时追加
    /// `"agentid"`，仅旧的第三方多应用套件需要填）。
    fn build_add_body(
        schedule: &WxCpOaSchedule,
        agent_id: Option<i32>,
    ) -> Result<String, WxErrorException> {
        let schedule_value =
            serde_json::to_value(schedule).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let body = match agent_id {
            Some(agent_id) => {
                serde_json::json!({ "schedule": schedule_value, "agentid": agent_id })
            }
            None => serde_json::json!({ "schedule": schedule_value }),
        };
        Ok(body.to_string())
    }

    /// 构造更新日程请求体（对应 Java `update` 内的
    /// `ImmutableMap.of("schedule", schedule)`）。
    fn build_update_body(schedule: &WxCpOaSchedule) -> Result<String, WxErrorException> {
        let schedule_value =
            serde_json::to_value(schedule).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(serde_json::json!({ "schedule": schedule_value }).to_string())
    }

    /// 构造获取日程详情请求体（对应 Java `getDetails` 内的
    /// `ImmutableMap.of("schedule_id_list", scheduleIds)`）。
    fn build_get_details_body(schedule_ids: &[&str]) -> String {
        serde_json::json!({ "schedule_id_list": schedule_ids }).to_string()
    }

    /// 构造删除日程请求体（对应 Java `delete` 内的
    /// `ImmutableMap.of("schedule_id", scheduleId)`）。
    fn build_delete_body(schedule_id: &str) -> String {
        serde_json::json!({ "schedule_id": schedule_id }).to_string()
    }

    /// 构造获取日历下的日程列表请求体（对应 Java `listByCalendar` 内的
    /// `Map`：`cal_id` 必有，`offset`/`limit` 非空才放入）。
    fn build_list_by_calendar_body(
        cal_id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> String {
        let mut body = serde_json::json!({ "cal_id": cal_id });
        if let Some(offset) = offset {
            body["offset"] = serde_json::json!(offset);
        }
        if let Some(limit) = limit {
            body["limit"] = serde_json::json!(limit);
        }
        body.to_string()
    }

    /// 从响应中解析 `schedule_list` 数组（对应 Java `getDetails`/
    /// `listByCalendar` 内 `GsonParser.parse(response).get("schedule_list")`
    /// + `TypeToken<List<WxCpOaSchedule>>`）。
    fn parse_schedule_list<T: serde::de::DeserializeOwned>(
        response: &str,
    ) -> Result<Vec<T>, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("schedule_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "schedule_list 字段缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpOaScheduleService for WxCpOaScheduleServiceImpl {
    async fn add(
        &self,
        schedule: &WxCpOaSchedule,
        agent_id: Option<i32>,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `add`：`POST SCHEDULE_ADD`，直接返回响应内容
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SCHEDULE_ADD);
        let body = Self::build_add_body(schedule, agent_id)?;
        svc.post(&api_url, &body).await
    }

    async fn update(&self, schedule: &WxCpOaSchedule) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：`POST SCHEDULE_UPDATE`（更新操作是覆盖式）
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SCHEDULE_UPDATE);
        let body = Self::build_update_body(schedule)?;
        svc.post(&api_url, &body).await?;
        Ok(())
    }

    async fn get_details(
        &self,
        schedule_ids: &[&str],
    ) -> Result<Vec<WxCpOaSchedule>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getDetails`：`POST SCHEDULE_GET`，解析 `schedule_list`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SCHEDULE_GET);
        let response = svc
            .post(&api_url, &Self::build_get_details_body(schedule_ids))
            .await?;
        Self::parse_schedule_list(&response)
    }

    async fn delete(&self, schedule_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete`：`POST SCHEDULE_DEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SCHEDULE_DEL);
        svc.post(&api_url, &Self::build_delete_body(schedule_id))
            .await?;
        Ok(())
    }

    async fn list_by_calendar(
        &self,
        cal_id: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<WxCpOaSchedule>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listByCalendar`：`POST SCHEDULE_LIST`，解析 `schedule_list`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SCHEDULE_LIST);
        let response = svc
            .post(
                &api_url,
                &Self::build_list_by_calendar_body(cal_id, offset, limit),
            )
            .await?;
        Self::parse_schedule_list(&response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `add`：无 `agentId` 时仅 `{"schedule":{...}}`，不携带
    /// `agentid` 字段。
    #[test]
    fn test_build_add_body_without_agent_id() {
        let schedule = WxCpOaSchedule {
            schedule_id: "s1".to_string(),
            ..Default::default()
        };
        let body = WxCpOaScheduleServiceImpl::build_add_body(&schedule, None).expect("构造失败");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert!(json.get("agentid").is_none());
        assert_eq!(json["schedule"]["schedule_id"], "s1");
    }

    /// Java `add`：带 `agentId` 时请求体含 `"agentid"`。
    #[test]
    fn test_build_add_body_with_agent_id() {
        let schedule = WxCpOaSchedule::default();
        let body =
            WxCpOaScheduleServiceImpl::build_add_body(&schedule, Some(3010040)).expect("构造失败");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["agentid"], 3010040);
    }

    /// Java `getDetails`：请求体 `{"schedule_id_list":["s1","s2"]}`。
    #[test]
    fn test_build_get_details_body() {
        assert_eq!(
            WxCpOaScheduleServiceImpl::build_get_details_body(&["s1", "s2"]),
            r#"{"schedule_id_list":["s1","s2"]}"#
        );
    }

    /// Java `listByCalendar`：`offset`/`limit` 为空时不放入请求体。
    #[test]
    fn test_build_list_by_calendar_body() {
        let body = WxCpOaScheduleServiceImpl::build_list_by_calendar_body("c1", None, None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["cal_id"], "c1");
        assert!(json.get("offset").is_none());
        assert!(json.get("limit").is_none());

        let body = WxCpOaScheduleServiceImpl::build_list_by_calendar_body("c1", Some(10), Some(20));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 20);
    }

    /// Java `getDetails`：响应 `schedule_list` 解析为列表。
    #[test]
    fn test_parse_schedule_list() {
        let response = r#"{"errcode":0,"errmsg":"ok","schedule_list":[{"schedule_id":"s1","summary":"日程一"}]}"#;
        let list = WxCpOaScheduleServiceImpl::parse_schedule_list::<WxCpOaSchedule>(response)
            .expect("解析失败");
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].schedule_id, "s1");
    }
}
