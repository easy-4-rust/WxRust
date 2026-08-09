//! 企业微信互联企业服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.corpgroup.service.impl.WxCpLinkedCorpServiceImpl`：
//! 以 `Weak<dyn WxCpCgService>` 持有集团服务（Java `@RequiredArgsConstructor`
//! 注入 `cpCgService`），经 `cpCgService.post(url, json, req)` 执行通道
//! 携带目标企业 access_token。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpCorpGroupCorpGetTokenReq, WxCpLinkedCorpAgentPerm, WxCpLinkedCorpDepartment,
    WxCpLinkedCorpUser,
};
use crate::corpgroup::service::{WxCpCgService, WxCpLinkedCorpService};
use crate::enums::url_linked_corp;

/// 企业微信互联企业服务实现。
pub struct WxCpLinkedCorpServiceImpl {
    cg_service: Weak<dyn WxCpCgService>,
}

impl WxCpLinkedCorpServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpCgService`；Weak 打破循环）。
    pub fn new(cg_service: Weak<dyn WxCpCgService>) -> Self {
        Self { cg_service }
    }

    fn cg_service(&self) -> Result<Arc<dyn WxCpCgService>, WxErrorException> {
        self.cg_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpCgService 引用已失效"))
    }

    /// 从响应解析嵌套数组字段（对应 Java
    /// `WxCpGsonBuilder.fromJson(tmpJson.get(field), TypeToken<List<..>>)`）。
    fn parse_list<T: serde::de::DeserializeOwned>(
        &self,
        response_content: &str,
        field: &str,
    ) -> Result<Vec<T>, WxErrorException> {
        let tmp: serde_json::Value = serde_json::from_str(response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = tmp
            .get(field)
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        serde_json::from_value(list)
            .map_err(|e| WxErrorException::Serde(format!("{field} 解析失败: {e}")))
    }
}

#[async_trait]
impl WxCpLinkedCorpService for WxCpLinkedCorpServiceImpl {
    async fn get_linked_corp_agent_perm(
        &self,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<WxCpLinkedCorpAgentPerm, WxErrorException> {
        let cg = self.cg_service()?;
        let config = cg.wx_cp_corp_group_config_storage();
        let url = config.api_url(url_linked_corp::GET_PERM_LIST);
        let body = serde_json::json!({}).to_string();
        let response_content = cg.post(&url, &body, req).await?;
        serde_json::from_str(&response_content).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_linked_corp_user(
        &self,
        user_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<WxCpLinkedCorpUser, WxErrorException> {
        let cg = self.cg_service()?;
        let config = cg.wx_cp_corp_group_config_storage();
        let url = config.api_url(url_linked_corp::GET_USER);
        let body = serde_json::json!({ "userid": user_id }).to_string();
        let response_content = cg.post(&url, &body, req).await?;
        // Java：从响应的 user_info 对象解析
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let user_info = tmp
            .get("user_info")
            .ok_or_else(|| WxErrorException::from_code(-99, "user_info 字段缺失"))?;
        serde_json::from_value(user_info.clone())
            .map_err(|e| WxErrorException::Serde(format!("user_info 解析失败: {e}")))
    }

    async fn get_linked_corp_simple_user_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpUser>, WxErrorException> {
        let cg = self.cg_service()?;
        let config = cg.wx_cp_corp_group_config_storage();
        let url = config.api_url(url_linked_corp::GET_USER_SIMPLELIST);
        let body = serde_json::json!({ "department_id": department_id }).to_string();
        let response_content = cg.post(&url, &body, req).await?;
        self.parse_list(&response_content, "userlist")
    }

    async fn get_linked_corp_user_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpUser>, WxErrorException> {
        let cg = self.cg_service()?;
        let config = cg.wx_cp_corp_group_config_storage();
        let url = config.api_url(url_linked_corp::GET_USER_LIST);
        let body = serde_json::json!({ "department_id": department_id }).to_string();
        let response_content = cg.post(&url, &body, req).await?;
        self.parse_list(&response_content, "userlist")
    }

    async fn get_linked_corp_department_list(
        &self,
        department_id: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<Vec<WxCpLinkedCorpDepartment>, WxErrorException> {
        let cg = self.cg_service()?;
        let config = cg.wx_cp_corp_group_config_storage();
        let url = config.api_url(url_linked_corp::GET_DEPARTMENT_LIST);
        let body = serde_json::json!({ "department_id": department_id }).to_string();
        let response_content = cg.post(&url, &body, req).await?;
        self.parse_list(&response_content, "department_list")
    }
}
