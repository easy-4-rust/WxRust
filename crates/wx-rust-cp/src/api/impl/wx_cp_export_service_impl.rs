//! 异步导出服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpExportServiceImpl`：以
//! `Weak<dyn WxCpService>` 持有门面（Java `@RequiredArgsConstructor` 注入
//! `mainService`），全部方法经门面 `get`/`post` 执行引擎发起请求。
//!
//! 语义镜像要点：
//! - `simpleUser`/`user`/`department`/`tagUser` 均 POST 对应导出路径，
//!   响应提取 `jobid`（对应 Java 私有 `export(String, WxCpExportRequest)`）；
//! - `getResult` 的 URL 以 `%s` 占位符替换 `jobid`（对应 Java
//!   `String.format(getApiUrl(GET_RESULT), jobId)`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpExportService, WxCpService};
use crate::bean::{WxCpExportRequest, WxCpExportResult};
use crate::enums::url_export;

/// 异步导出服务实现。
pub struct WxCpExportServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpExportServiceImpl {
    /// 构建异步导出服务（对应 Java 构造器注入 `WxCpService`）。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 升级门面引用（对应 Java 直接持有的 `mainService` 字段；Weak 引用
    /// 失效时抛 -99，ADAPTED）。
    fn service(&self) -> Result<Arc<dyn WxCpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))
    }

    /// 发起导出任务并提取 jobid（对应 Java 私有 `export` 方法）。
    async fn export(
        &self,
        path: &str,
        params: &WxCpExportRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.service()?;
        let body = params.to_json().map_err(WxErrorException::Serde)?;
        let url = svc.wx_cp_config_storage().api_url(path);
        let response = svc.post(&url, &body).await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("jobid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "jobid 字段缺失"))
    }
}

#[async_trait]
impl WxCpExportService for WxCpExportServiceImpl {
    /// 导出成员（对应 Java `simpleUser`）。
    async fn simple_user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException> {
        self.export(url_export::SIMPLE_USER, params).await
    }

    /// 导出成员详情（对应 Java `user`）。
    async fn user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException> {
        self.export(url_export::USER, params).await
    }

    /// 导出部门（对应 Java `department`）。
    async fn department(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException> {
        self.export(url_export::DEPARTMENT, params).await
    }

    /// 导出标签成员（对应 Java `tagUser`）。
    async fn tag_user(&self, params: &WxCpExportRequest) -> Result<String, WxErrorException> {
        self.export(url_export::TAG_USER, params).await
    }

    /// 获取导出结果（对应 Java `getResult`）。
    async fn get_result(&self, job_id: &str) -> Result<WxCpExportResult, WxErrorException> {
        let svc = self.service()?;
        // Java：String.format(getApiUrl(GET_RESULT), jobId)
        let url = svc
            .wx_cp_config_storage()
            .api_url(url_export::GET_RESULT)
            .replacen("%s", job_id, 1);
        let response = svc.get(&url, "").await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    //! 内嵌测试：导出任务发起（jobid 提取）与导出结果获取（路径 %s
    //! 占位符替换 + 响应解析）。

    use super::*;
    use crate::api::r#impl::g2_impls::test_support::{
        MockServer, dispatch, json, service_with_host, weak_service,
    };

    /// 镜像 Java `testUser`/`testGetResult`：导出成员详情提取 jobid；
    /// 获取导出结果按 jobid 拼路径并解析 data_list。
    #[tokio::test]
    async fn test_export_user_and_get_result() {
        let server = MockServer::start(dispatch(|path| {
            if path.contains("/cgi-bin/export/user") {
                json(r#"{"errcode":0,"errmsg":"ok","jobid":"jobid_123"}"#)
            } else if path.contains("/cgi-bin/export/get_result") {
                json(
                    r#"{"errcode":0,"errmsg":"ok","status":1,"data_list":[{"url":"https://export/1.csv","size":1024}]}"#,
                )
            } else {
                json(r#"{"errcode":0,"errmsg":"ok"}"#)
            }
        }))
        .await;
        let service = service_with_host(&server.url(""));
        let svc_impl = WxCpExportServiceImpl::new(weak_service(&service));

        let mut params = WxCpExportRequest::default();
        params.block_size = 10000;
        let job_id = svc_impl.user(&params).await.expect("导出成员成功");
        assert_eq!(job_id, "jobid_123");
        assert!(server.last_path().contains("/cgi-bin/export/user"));
        assert!(server.last_path().contains("access_token=MOCK_TOKEN"));
        assert!(
            server.last_body().contains(r#""block_size":10000"#),
            "body: {}",
            server.last_body()
        );

        let result = svc_impl
            .get_result(&job_id)
            .await
            .expect("获取导出结果成功");
        assert_eq!(result.status, 1);
        assert_eq!(result.data_list.len(), 1);
        assert_eq!(result.data_list[0].url, "https://export/1.csv");
        assert!(
            server
                .last_path()
                .contains("/cgi-bin/export/get_result?jobid=jobid_123"),
            "path: {}",
            server.last_path()
        );
    }
}
