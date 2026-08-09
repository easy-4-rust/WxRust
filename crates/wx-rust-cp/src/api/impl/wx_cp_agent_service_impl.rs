//! 企业号应用管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpAgentServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::api::wx_consts::ERR_CODE;
use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, WxErrorException};

use crate::api::{WxCpAgentService, WxCpService};
use crate::bean::{WxCpAgent, WxCpTpAdmin};
use crate::enums::url_agent::agent::*;

/// 企业号应用管理服务实现。
pub struct WxCpAgentServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpAgentServiceImpl {
    /// 构建应用管理服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpAgentService for WxCpAgentServiceImpl {
    async fn get(&self, agent_id: i32) -> Result<WxCpAgent, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get`：GET `String.format(AGENT_GET, agentId)`（`%d` 替换），
        // 整体响应 `WxCpAgent.fromJson`
        let config = svc.wx_cp_config_storage();
        let url = config
            .api_url(AGENT_GET)
            .replace("%d", &agent_id.to_string());
        let response_content = svc.get(&url, "").await?;
        WxCpAgent::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn set(&self, agent_info: &WxCpAgent) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `set`：POST `AGENT_SET`，请求体 `agentInfo.toJson()`；
        // errcode != 0 时抛 `WxErrorException(WxError.fromJson(...))`
        let config = svc.wx_cp_config_storage();
        let body = agent_info.to_json().map_err(WxErrorException::Serde)?;
        let response_content = svc.post(&config.api_url(AGENT_SET), &body).await?;
        check_err_code(&response_content)
    }

    async fn list(&self) -> Result<Vec<WxCpAgent>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `list`：GET `AGENT_LIST`；errcode != 0 抛错，否则解析
        // `agentlist` 数组
        let config = svc.wx_cp_config_storage();
        let response_content = svc.get(&config.api_url(AGENT_LIST), "").await?;
        check_err_code(&response_content)?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("agentlist")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| WxErrorException::from_code(-99, "agentlist 字段缺失"))?;
        list.iter()
            .map(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| WxErrorException::Serde(e.to_string()))
            })
            .collect()
    }

    async fn get_admin_list(&self, agent_id: i32) -> Result<WxCpTpAdmin, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getAdminList`：POST `AGENT_GET_ADMIN_LIST` `{"agentid":...}`；
        // errcode != 0 抛错，否则整体响应 `WxCpTpAdmin.fromJson`
        let body = serde_json::json!({ "agentid": agent_id }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(AGENT_GET_ADMIN_LIST), &body)
            .await?;
        check_err_code(&response_content)?;
        WxCpTpAdmin::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}

/// 校验响应错误码（对应 Java
/// `jsonObject.get(ERR_CODE).getAsInt() != 0 → WxErrorException(WxError.fromJson)`；
/// 注：标准执行器已对 errcode != 0 抛错，此处为镜像 Java 显式二次校验）。
fn check_err_code(response_content: &str) -> Result<(), WxErrorException> {
    let json: serde_json::Value = serde_json::from_str(response_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
    let err_code = json
        .get(ERR_CODE)
        .and_then(serde_json::Value::as_i64)
        .unwrap_or(-1);
    if err_code != 0 {
        let error = WxError::from_json_with_type(response_content, Some(WxType::Cp));
        return Err(WxErrorException::from_code(
            error.error_code,
            error.error_msg.unwrap_or_default(),
        ));
    }
    Ok(())
}
