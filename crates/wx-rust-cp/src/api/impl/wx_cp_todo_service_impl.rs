//! 企业微信待办服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpTodoServiceImpl`：
//! 以 `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor`
//! 注入 `cpService`），全部方法经门面 `post` 执行引擎发起请求。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpService, WxCpTodoService};
use crate::bean::wx_cp_todo::{WxCpTodo, WxCpTodoAttendee};
use crate::enums::url_todo;

/// 企业微信待办服务实现。
pub struct WxCpTodoServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpTodoServiceImpl {
    /// 构建待办服务（对应 Java 构造器注入 `WxCpService`）。
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
}

#[async_trait]
impl WxCpTodoService for WxCpTodoServiceImpl {
    /// 获取待办详情（对应 Java `WxCpTodoServiceImpl.get`）。
    async fn get(&self, todo_id: &str) -> Result<WxCpTodo, WxErrorException> {
        let svc = self.service()?;
        let mut param = serde_json::Map::new();
        param.insert(
            "todo_id".to_string(),
            serde_json::Value::String(todo_id.to_string()),
        );
        let url = svc.wx_cp_config_storage().api_url(url_todo::TODO_GET);
        let body = serde_json::Value::Object(param).to_string();
        let response = svc.post(&url, &body).await?;
        WxCpTodo::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 更新待办状态（对应 Java `WxCpTodoServiceImpl.update`）。
    async fn update(
        &self,
        todo_id: &str,
        status: Option<i32>,
        attendees: Option<Vec<WxCpTodoAttendee>>,
    ) -> Result<(), WxErrorException> {
        let svc = self.service()?;
        let mut param = serde_json::Map::new();
        param.insert(
            "todo_id".to_string(),
            serde_json::Value::String(todo_id.to_string()),
        );
        if let Some(s) = status {
            param.insert("status".to_string(), serde_json::Value::Number(s.into()));
        }
        if let Some(ref att) = attendees {
            if !att.is_empty() {
                let json_att = serde_json::to_value(att)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                param.insert("attendees".to_string(), json_att);
            }
        }
        let url = svc.wx_cp_config_storage().api_url(url_todo::TODO_UPDATE);
        let body = serde_json::Value::Object(param).to_string();
        svc.post(&url, &body).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：待办获取/更新的请求路径/请求体/响应解析。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testGet`：请求路径与请求体、响应解析。
    #[tokio::test]
    async fn test_todo_get() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/todo/get") {
                json(r#"{"errcode":0,"errmsg":"ok","todo_id":"TODO_1","content":"完成报告","creator":"zhangsan","status":1,"create_time":1700000000,"attendees":[{"userid":"zhangsan","status":1}],"end_time":1700100000,"reminders":[{"remind_time":1700050000}]}"#)
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpTodoServiceImpl::new(weak_service(&service));

        let todo = svc_impl.get("TODO_1").await.expect("获取待办成功");
        assert_eq!(todo.todo_id.as_deref(), Some("TODO_1"));
        assert_eq!(todo.content.as_deref(), Some("完成报告"));
        assert_eq!(todo.status, Some(1));
        assert_eq!(todo.attendees.len(), 1);
        assert_eq!(todo.attendees[0].userid.as_deref(), Some("zhangsan"));
        assert!(
            server.last_path().contains("/cgi-bin/todo/get"),
            "path: {}",
            server.last_path()
        );
        let body = server.last_body();
        assert!(body.contains(r#""todo_id":"TODO_1""#), "body: {body}");
    }

    /// 镜像 Java `testUpdate`：请求路径与请求体。
    #[tokio::test]
    async fn test_todo_update() {
        let server =
            MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpTodoServiceImpl::new(weak_service(&service));

        // 更新状态 + 参与人
        let attendees = vec![WxCpTodoAttendee {
            userid: Some("zhangsan".to_string()),
            status: Some(0),
        }];
        svc_impl
            .update("TODO_1", Some(0), Some(attendees))
            .await
            .expect("更新待办成功");
        assert!(
            server.last_path().contains("/cgi-bin/todo/update"),
            "path: {}",
            server.last_path()
        );
        let body = server.last_body();
        assert!(body.contains(r#""todo_id":"TODO_1""#), "body: {body}");
        assert!(body.contains(r#""status":0"#), "body: {body}");
        assert!(body.contains(r#""userid":"zhangsan""#), "body: {body}");
    }

    /// 仅更新状态，不修改参与人。
    #[tokio::test]
    async fn test_todo_update_status_only() {
        let server =
            MockServer::start(dispatch(|_path| json(r#"{"errcode":0,"errmsg":"ok"}"#))).await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpTodoServiceImpl::new(weak_service(&service));

        svc_impl
            .update("TODO_2", Some(0), None)
            .await
            .expect("更新待办状态成功");
        let body = server.last_body();
        assert!(body.contains(r#""todo_id":"TODO_2""#), "body: {body}");
        assert!(body.contains(r#""status":0"#), "body: {body}");
        // 不应包含 attendees
        assert!(!body.contains("attendees"), "body: {body}");
    }
}
