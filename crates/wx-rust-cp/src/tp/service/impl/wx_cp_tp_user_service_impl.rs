//! 企业微信第三方应用成员服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpUserServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面；涉及 corpId 的方法使用授权
//! 企业的 access_token（`config.getAccessToken(corpId)`）。

use std::collections::HashMap;
use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpInviteResult, WxCpUser, WxCpUserExternalContactInfo};
use crate::enums::url_user;
use crate::tp::service::{WxCpTpService, WxCpTpUserService};

/// 企业微信第三方应用成员服务实现。
pub struct WxCpTpUserServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpUserServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 解析响应中的 `userlist` 为 `WxCpUser` 列表（对应 Java
    /// `WxCpGsonBuilder.fromJson(tmp.get("userlist"), TypeToken<List>)`）。
    fn parse_user_list(&self, response_content: &str) -> Result<Vec<WxCpUser>, WxErrorException> {
        let tmp: serde_json::Value = serde_json::from_str(response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = tmp
            .get("userlist")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        serde_json::from_value(list)
            .map_err(|e| WxErrorException::Serde(format!("userlist 解析失败: {e}")))
    }
}

#[async_trait]
impl WxCpTpUserService for WxCpTpUserServiceImpl {
    async fn authenticate(&self, user_id: &str) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(&format!("{}{user_id}", url_user::USER_AUTHENTICATE));
        service.get(&url, "").await?;
        Ok(())
    }

    async fn create(&self, user: &WxCpUser) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_user::USER_CREATE);
        let json = user.to_json().map_err(WxErrorException::Serde)?;
        service.post(&url, &json).await?;
        Ok(())
    }

    async fn update(&self, user: &WxCpUser) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_user::USER_UPDATE);
        let json = user.to_json().map_err(WxErrorException::Serde)?;
        service.post(&url, &json).await?;
        Ok(())
    }

    async fn delete(&self, user_ids: &[String]) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        if user_ids.len() == 1 {
            // 单用户走 GET 删除（对应 Java USER_DELETE + userIds[0]）
            let url = config.api_url(&format!("{}{}", url_user::USER_DELETE, user_ids[0]));
            service.get(&url, "").await?;
            return Ok(());
        }
        let body = serde_json::json!({ "useridlist": user_ids }).to_string();
        let url = config.api_url(url_user::USER_BATCH_DELETE);
        service.post(&url, &body).await?;
        Ok(())
    }

    async fn get_by_id(&self, user_id: &str, corp_id: &str) -> Result<WxCpUser, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = format!(
            "{}&access_token={}",
            config.api_url(&format!("{}{user_id}", url_user::USER_GET)),
            config.access_token(corp_id).unwrap_or_default()
        );
        let response_content = service.get(&url, "").await?;
        WxCpUser::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn list_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
        corp_id: &str,
    ) -> Result<Vec<WxCpUser>, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java：params 以 & 开头拼接到 URL（query 由执行器附加）
        let mut params = String::new();
        if let Some(fetch_child) = fetch_child {
            params.push_str(&format!(
                "&fetch_child={}",
                if fetch_child { "1" } else { "0" }
            ));
        }
        if let Some(status) = status {
            params.push_str(&format!("&status={status}"));
        } else {
            params.push_str("&status=0");
        }
        params.push_str(&format!(
            "&access_token={}",
            config.access_token(corp_id).unwrap_or_default()
        ));
        let url = config.api_url(&format!("{}{depart_id}", url_user::USER_LIST));
        let response_content = service.get(&url, &params).await?;
        self.parse_user_list(&response_content)
    }

    async fn list_simple_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
        corp_id: &str,
    ) -> Result<Vec<WxCpUser>, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java：params 为 query 本体（不带前导 &），且以 withoutSuiteToken
        // 通道执行
        let mut params = String::new();
        if let Some(fetch_child) = fetch_child {
            params.push_str(&format!(
                "fetch_child={}",
                if fetch_child { "1" } else { "0" }
            ));
        }
        if let Some(status) = status {
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str(&format!("status={status}"));
        } else {
            if !params.is_empty() {
                params.push('&');
            }
            params.push_str("status=0");
        }
        if !params.is_empty() {
            params.push('&');
        }
        params.push_str(&format!(
            "access_token={}",
            config.access_token(corp_id).unwrap_or_default()
        ));
        let url = config.api_url(&format!("{}{depart_id}", url_user::USER_SIMPLE_LIST));
        let response_content = service.get_without_suite_token(&url, &params, true).await?;
        self.parse_user_list(&response_content)
    }

    async fn list_simple_by_department_without_corp(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException> {
        // Java @Deprecated 重载：不带 corpId（suite token 通道）
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let mut params = String::new();
        if let Some(fetch_child) = fetch_child {
            params.push_str(&format!(
                "&fetch_child={}",
                if fetch_child { "1" } else { "0" }
            ));
        }
        if let Some(status) = status {
            params.push_str(&format!("&status={status}"));
        } else {
            params.push_str("&status=0");
        }
        let url = config.api_url(&format!("{}{depart_id}", url_user::USER_SIMPLE_LIST));
        let response_content = service.get(&url, &params).await?;
        self.parse_user_list(&response_content)
    }

    async fn invite(
        &self,
        user_ids: &[String],
        party_ids: &[String],
        tag_ids: &[String],
    ) -> Result<WxCpInviteResult, WxErrorException> {
        let service = self.service()?;
        let mut body = serde_json::Map::new();
        if !user_ids.is_empty() {
            body.insert(
                "user".to_string(),
                serde_json::Value::Array(
                    user_ids
                        .iter()
                        .map(|v| serde_json::Value::String(v.clone()))
                        .collect(),
                ),
            );
        }
        if !party_ids.is_empty() {
            body.insert(
                "party".to_string(),
                serde_json::Value::Array(
                    party_ids
                        .iter()
                        .map(|v| serde_json::Value::String(v.clone()))
                        .collect(),
                ),
            );
        }
        if !tag_ids.is_empty() {
            body.insert(
                "tag".to_string(),
                serde_json::Value::Array(
                    tag_ids
                        .iter()
                        .map(|v| serde_json::Value::String(v.clone()))
                        .collect(),
                ),
            );
        }
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_user::BATCH_INVITE);
        let response = service
            .post(&url, &serde_json::Value::Object(body).to_string())
            .await?;
        WxCpInviteResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn user_id_2_openid(
        &self,
        user_id: &str,
        agent_id: Option<i32>,
    ) -> Result<HashMap<String, String>, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_user::USER_CONVERT_TO_OPENID);
        let mut body = serde_json::Map::new();
        body.insert(
            "userid".to_string(),
            serde_json::Value::String(user_id.to_string()),
        );
        if let Some(agent_id) = agent_id {
            body.insert(
                "agentid".to_string(),
                serde_json::Value::Number(agent_id.into()),
            );
        }
        let response_content = service
            .post(&url, &serde_json::Value::Object(body).to_string())
            .await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = HashMap::new();
        if let Some(openid) = tmp.get("openid").and_then(|v| v.as_str()) {
            result.insert("openid".to_string(), openid.to_string());
        }
        if let Some(appid) = tmp.get("appid").and_then(|v| v.as_str()) {
            result.insert("appid".to_string(), appid.to_string());
        }
        Ok(result)
    }

    async fn openid_2_user_id(&self, openid: &str) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_user::USER_CONVERT_TO_USERID);
        let body = serde_json::json!({ "openid": openid }).to_string();
        let response_content = service.post(&url, &body).await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        tmp.get("userid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "userid 字段缺失"))
    }

    async fn get_user_id(&self, mobile: &str, corp_id: &str) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let body = serde_json::json!({ "mobile": mobile }).to_string();
        let url = format!(
            "{}?access_token={}",
            config.api_url(url_user::GET_USER_ID),
            config.access_token(corp_id).unwrap_or_default()
        );
        let response_content = service.post(&url, &body).await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        tmp.get("userid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "userid 字段缺失"))
    }

    async fn get_external_contact(
        &self,
        user_id: &str,
    ) -> Result<WxCpUserExternalContactInfo, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(&format!("{}{user_id}", url_user::GET_EXTERNAL_CONTACT));
        let response_content = service.get(&url, "").await?;
        WxCpUserExternalContactInfo::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}
