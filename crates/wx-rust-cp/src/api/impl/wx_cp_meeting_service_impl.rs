//! 企业微信会议服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpMeetingServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `cpService`），全部方法经门面 `post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - `create` 直接返回响应原文（对应 Java
//!   `return this.cpService.post(...)`——响应即会议 ID）；
//! - `cancel`/`getDetail` 请求体为 `{"meetingid": ...}`（对应 Java
//!   `ImmutableMap.of("meetingid", meetingId)`）；
//! - `getUserMeetingIds` 仅 `userid` 恒写入，cursor/limit/begin_time/
//!   end_time 按 Java `if (xx != null)` 分支写入。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpMeetingService, WxCpService};
use crate::bean::{WxCpMeeting, WxCpMeetingUpdateResult, WxCpUserMeetingIdResult};
use crate::enums::url_oa;

/// 序列化 JSON 对象为请求体字符串（`serde_json::Map` 无 `Display`，以
/// `Value::Object` 包装后序列化，对应 Java `JsonObject.toString()`）。
fn map_to_string(obj: &serde_json::Map<String, serde_json::Value>) -> String {
    serde_json::Value::Object(obj.clone()).to_string()
}

/// 企业微信会议服务实现。
pub struct WxCpMeetingServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpMeetingServiceImpl {
    /// 构建会议服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `cpService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 序列化请求对象（对应 Java `WxCpGsonBuilder.toJson`）。
    fn to_json<T: serde::Serialize>(value: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(value).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpMeetingService for WxCpMeetingServiceImpl {
    /// 创建预约会议（对应 Java `create`）。
    async fn create(&self, meeting: &WxCpMeeting) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::MEETING_ADD);
        // Java：直接返回响应内容（会议 ID）
        svc.post(&url, &Self::to_json(meeting)?).await
    }

    /// 修改预约会议（对应 Java `update`）。
    async fn update(
        &self,
        meeting: &WxCpMeeting,
    ) -> Result<WxCpMeetingUpdateResult, WxErrorException> {
        let svc = self.service()?;
        let url = svc.wx_cp_config_storage().api_url(url_oa::MEETING_UPDATE);
        let response = svc.post(&url, &Self::to_json(meeting)?).await?;
        WxCpMeetingUpdateResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 取消预约会议（对应 Java `cancel`）。
    async fn cancel(&self, meeting_id: &str) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "meetingid".to_string(),
            serde_json::Value::String(meeting_id.to_string()),
        );
        let url = svc.wx_cp_config_storage().api_url(url_oa::MEETING_CANCEL);
        svc.post(&url, &map_to_string(&obj)).await?;
        Ok(())
    }

    /// 获取会议详情（对应 Java `getDetail`）。
    async fn get_detail(&self, meeting_id: &str) -> Result<WxCpMeeting, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "meetingid".to_string(),
            serde_json::Value::String(meeting_id.to_string()),
        );
        let url = svc.wx_cp_config_storage().api_url(url_oa::MEETING_DETAIL);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取成员会议 ID 列表（对应 Java `getUserMeetingIds`）。
    async fn get_user_meeting_ids(
        &self,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
        begin_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<WxCpUserMeetingIdResult, WxErrorException> {
        let svc = self.service()?;
        let mut obj = serde_json::Map::new();
        obj.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        if let Some(cursor) = cursor {
            obj.insert(
                "cursor".to_string(),
                serde_json::Value::String(cursor.to_string()),
            );
        }
        if let Some(limit) = limit {
            obj.insert("limit".to_string(), serde_json::Value::from(limit));
        }
        if let Some(begin_time) = begin_time {
            obj.insert(
                "begin_time".to_string(),
                serde_json::Value::from(begin_time),
            );
        }
        if let Some(end_time) = end_time {
            obj.insert("end_time".to_string(), serde_json::Value::from(end_time));
        }
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_USER_MEETING_ID);
        let response = svc.post(&url, &map_to_string(&obj)).await?;
        WxCpUserMeetingIdResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：会议创建（返回响应原文）/详情（响应解析）与取消请求体。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testCreate`/`testGetDetail`/`testCancel`：create 直接
    /// 返回响应原文（会议 ID）；getDetail 解析 WxCpMeeting；cancel 请求体
    /// 为 {"meetingid": ...}。
    #[tokio::test]
    async fn test_meeting_create_get_detail() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/meeting/create") {
                json(r#"{"errcode":0,"errmsg":"ok","meetingid":"MEETING_123"}"#)
            } else if path.contains("/cgi-bin/meeting/get_info") {
                json(r#"{"errcode":0,"errmsg":"ok","meetingid":"MEETING_123","title":"周会","meeting_start":1600000000,"meeting_duration":3600}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpMeetingServiceImpl::new(weak_service(&service));

        // create：返回响应原文（对应 Java 直接 return post(...)）
        let mut meeting = WxCpMeeting::default();
        meeting.title = "周会".to_string();
        meeting.meeting_duration = 3600;
        let response = svc_impl.create(&meeting).await.expect("创建会议成功");
        assert!(response.contains("MEETING_123"), "response: {response}");
        assert!(server.last_path().contains("/cgi-bin/meeting/create"));
        assert!(
            server.last_body().contains(r#""title":"周会""#),
            "body: {}",
            server.last_body()
        );

        // getDetail：解析 WxCpMeeting
        let detail = svc_impl
            .get_detail("MEETING_123")
            .await
            .expect("获取会议详情成功");
        assert_eq!(detail.meeting_id, "MEETING_123");
        assert_eq!(detail.title, "周会");
        assert_eq!(detail.meeting_duration, 3600);
        let body = server.last_body();
        assert!(
            body.contains(r#""meetingid":"MEETING_123""#),
            "body: {body}"
        );

        // cancel：{"meetingid": ...}
        svc_impl.cancel("MEETING_123").await.expect("取消会议成功");
        assert!(server.last_path().contains("/cgi-bin/meeting/cancel"));
        assert!(
            server.last_body().contains(r#""meetingid":"MEETING_123""#),
            "body: {}",
            server.last_body()
        );
    }
}
