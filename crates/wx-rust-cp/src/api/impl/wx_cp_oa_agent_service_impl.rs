//! 企业微信自建应用服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaAgentServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaAgentService, WxCpService};
use crate::bean::WxCpOpenApprovalData;
use crate::enums::url_oa;

/// 企业微信自建应用服务实现。
pub struct WxCpOaAgentServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaAgentServiceImpl {
    /// 构建自建应用服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造查询审批申请当前状态请求体（对应 Java `getOpenApprovalData`
    /// 内的 `JsonObject`：`{"thirdNo": thirdNo}`）。
    fn build_get_open_approval_data_body(third_no: &str) -> String {
        serde_json::json!({ "thirdNo": third_no }).to_string()
    }
}

#[async_trait]
impl WxCpOaAgentService for WxCpOaAgentServiceImpl {
    async fn get_open_approval_data(
        &self,
        third_no: &str,
    ) -> Result<WxCpOpenApprovalData, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getOpenApprovalData`：`POST GET_OPEN_APPROVAL_DATA`，响应
        // `data` 字段为 `WxCpOpenApprovalData`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::GET_OPEN_APPROVAL_DATA);
        let response_content = svc
            .post(&api_url, &Self::build_get_open_approval_data_body(third_no))
            .await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let data = json
            .get("data")
            .ok_or_else(|| WxErrorException::from_code(-99, "data 字段缺失"))?;
        WxCpOpenApprovalData::from_json(&data.to_string()).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::config::WxCpConfigStorage;

    use super::*;

    /// 仅用于验证服务引用释放（Weak 未持有）路径的桩实现；该路径在
    /// `upgrade()` 失败后即返回 -99，不会触碰配置存储。
    struct MockWxCpService {
        client: reqwest::Client,
    }

    impl WxCpService for MockWxCpService {
        fn wx_cp_config_storage(&self) -> Arc<dyn WxCpConfigStorage> {
            unreachable!("released-service 路径不会访问配置存储")
        }

        fn http_client(&self) -> &reqwest::Client {
            &self.client
        }
    }

    /// Java `getOpenApprovalData`：请求体 `{"thirdNo": "..."}`。
    #[test]
    fn test_build_get_open_approval_data_body() {
        assert_eq!(
            WxCpOaAgentServiceImpl::build_get_open_approval_data_body("20200101001"),
            r#"{"thirdNo":"20200101001"}"#
        );
    }

    /// 服务引用已释放（Weak 未持有）时返回错误码 -99。
    #[tokio::test]
    async fn test_get_open_approval_data_service_released() {
        let arc: Arc<dyn WxCpService> = Arc::new(MockWxCpService {
            client: reqwest::Client::new(),
        });
        let svc = WxCpOaAgentServiceImpl::new(Arc::downgrade(&arc));
        drop(arc);
        let err = svc.get_open_approval_data("20200101001").await.unwrap_err();
        assert_eq!(err.error_code(), Some(-99));
    }
}
