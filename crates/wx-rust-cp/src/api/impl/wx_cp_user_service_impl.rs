//! 成员服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpUserServiceImpl`。
//! 各方法经门面 `WxCpService` 的 `get`/`post` 通道（执行引擎自动注入
//! access_token + 重试 + token 自动刷新）发起请求，响应解析镜像 Java
//! Gson 逻辑。

use std::collections::HashMap;
use std::sync::Weak;

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpService, WxCpUserService};
use crate::bean::{
    WxCpDeptUserResult, WxCpExternalContactInfo, WxCpInviteResult, WxCpOpenUseridToUseridResult,
    WxCpUser, WxCpUseridToOpenUseridResult,
};
use crate::enums::url_user::*;

/// 成员服务实现。
pub struct WxCpUserServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpUserServiceImpl {
    /// 构建成员服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpUserService for WxCpUserServiceImpl {
    async fn authenticate(&self, user_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `authenticate`：GET `USER_AUTHENTICATE + userId`，忽略响应
        let config = svc.wx_cp_config_storage();
        svc.get(&config.api_url(USER_AUTHENTICATE), user_id).await?;
        Ok(())
    }

    async fn list_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listByDepartment`：params 拼接 `&fetch_child=`/`&status=`
        // （status 缺省补 `&status=0`），GET `USER_LIST + departId`
        let params = build_department_params(fetch_child, status);
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{depart_id}", config.api_url(USER_LIST));
        let response_content = svc.get(&url, &params).await?;
        parse_user_list(&response_content)
    }

    async fn list_simple_by_department(
        &self,
        depart_id: i64,
        fetch_child: Option<bool>,
        status: Option<i32>,
    ) -> Result<Vec<WxCpUser>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listSimpleByDepartment`：与 `listByDepartment` 同构，
        // 仅 URL 换 `USER_SIMPLE_LIST`
        let params = build_department_params(fetch_child, status);
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{depart_id}", config.api_url(USER_SIMPLE_LIST));
        let response_content = svc.get(&url, &params).await?;
        parse_user_list(&response_content)
    }

    async fn create(&self, user: &WxCpUser) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create`：POST `USER_CREATE`，请求体 `user.toJson()`
        let config = svc.wx_cp_config_storage();
        let body = user.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&config.api_url(USER_CREATE), &body).await?;
        Ok(())
    }

    async fn update(&self, user: &WxCpUser) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：POST `USER_UPDATE`，请求体 `user.toJson()`
        let config = svc.wx_cp_config_storage();
        let body = user.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&config.api_url(USER_UPDATE), &body).await?;
        Ok(())
    }

    async fn delete(&self, user_ids: &[&str]) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        let config = svc.wx_cp_config_storage();
        if user_ids.len() == 1 {
            // Java：单成员走 GET `USER_DELETE + userId`
            let url = format!("{}{}", config.api_url(USER_DELETE), user_ids[0]);
            svc.get(&url, "").await?;
            return Ok(());
        }
        // Java：多成员走 POST `USER_BATCH_DELETE`，请求体 `{"useridlist":[...]}`
        let userid_list: Vec<serde_json::Value> = user_ids
            .iter()
            .map(|id| serde_json::Value::from(*id))
            .collect();
        let body = serde_json::json!({ "useridlist": userid_list }).to_string();
        svc.post(&config.api_url(USER_BATCH_DELETE), &body).await?;
        Ok(())
    }

    async fn get_by_id(&self, user_id: &str) -> Result<WxCpUser, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getById`：GET `USER_GET + userid`，响应整体 `WxCpUser.fromJson`
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{user_id}", config.api_url(USER_GET));
        let response_content = svc.get(&url, "").await?;
        WxCpUser::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn invite(
        &self,
        user_ids: &[&str],
        party_ids: &[&str],
        tag_ids: &[&str],
    ) -> Result<WxCpInviteResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `invite`：POST `BATCH_INVITE`，`user`/`party`/`tag` 数组
        // 按非 null 条件追加（Rust 以空切片判空，ADAPTED：无 null 语义）
        let mut body = serde_json::Map::new();
        if !user_ids.is_empty() {
            body.insert(
                "user".to_string(),
                serde_json::Value::Array(
                    user_ids
                        .iter()
                        .map(|v| serde_json::Value::from(*v))
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
                        .map(|v| serde_json::Value::from(*v))
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
                        .map(|v| serde_json::Value::from(*v))
                        .collect(),
                ),
            );
        }
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(
                &config.api_url(BATCH_INVITE),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        WxCpInviteResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn user_id2_openid(
        &self,
        user_id: &str,
        agent_id: Option<i32>,
    ) -> Result<HashMap<String, String>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `userId2Openid`：POST `USER_CONVERT_TO_OPENID`，
        // `agentid` 非必填；响应 map 按字段存在性组装（openid/appid）
        let mut body = serde_json::Map::new();
        body.insert("userid".to_string(), serde_json::Value::from(user_id));
        if let Some(agent_id) = agent_id {
            body.insert("agentid".to_string(), serde_json::Value::from(agent_id));
        }
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(
                &config.api_url(USER_CONVERT_TO_OPENID),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let mut result = HashMap::new();
        if let Some(v) = json.get("openid").and_then(serde_json::Value::as_str) {
            result.insert("openid".to_string(), v.to_string());
        }
        if let Some(v) = json.get("appid").and_then(serde_json::Value::as_str) {
            result.insert("appid".to_string(), v.to_string());
        }
        Ok(result)
    }

    async fn openid2_user_id(&self, openid: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `openid2UserId`：POST `USER_CONVERT_TO_USERID`，取 `userid`
        let body = serde_json::json!({ "openid": openid }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(USER_CONVERT_TO_USERID), &body)
            .await?;
        parse_string_field(&response_content, "userid")
    }

    async fn get_user_id(&self, mobile: &str) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserId`：POST `GET_USER_ID` `{"mobile":...}`，取 `userid`
        let body = serde_json::json!({ "mobile": mobile }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc.post(&config.api_url(GET_USER_ID), &body).await?;
        parse_string_field(&response_content, "userid")
    }

    async fn get_user_id_by_email(
        &self,
        email: &str,
        email_type: i32,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserIdByEmail`：POST `GET_USER_ID_BY_EMAIL`
        // `{"email":..., "email_type":...}`，取 `userid`
        let body = serde_json::json!({ "email": email, "email_type": email_type }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(GET_USER_ID_BY_EMAIL), &body)
            .await?;
        parse_string_field(&response_content, "userid")
    }

    async fn get_external_contact(
        &self,
        user_id: &str,
    ) -> Result<WxCpExternalContactInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getExternalContact`：GET `GET_EXTERNAL_CONTACT + userId`
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{user_id}", config.api_url(GET_EXTERNAL_CONTACT));
        let response_content = svc.get(&url, "").await?;
        WxCpExternalContactInfo::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn get_join_qr_code(&self, size_type: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getJoinQrCode`：GET `GET_JOIN_QR_CODE + sizeType`，取 `join_qrcode`
        let config = svc.wx_cp_config_storage();
        let url = format!("{}{size_type}", config.api_url(GET_JOIN_QR_CODE));
        let response_content = svc.get(&url, "").await?;
        parse_string_field(&response_content, "join_qrcode")
    }

    async fn get_active_stat(&self, date: DateTime<Utc>) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getActiveStat`：POST `GET_ACTIVE_STAT`，
        // `date` 以 `yyyy-MM-dd` 格式化（FastDateFormat），取 `active_cnt`
        let date_str = date.format("%Y-%m-%d").to_string();
        let body = serde_json::json!({ "date": date_str }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc.post(&config.api_url(GET_ACTIVE_STAT), &body).await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("active_cnt")
            .and_then(serde_json::Value::as_i64)
            .map(|v| v as i32)
            .ok_or_else(|| WxErrorException::from_code(-99, "active_cnt 字段缺失"))
    }

    async fn userid_to_open_userid(
        &self,
        userid_list: &[&str],
    ) -> Result<WxCpUseridToOpenUseridResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `useridToOpenUserid`：POST `USERID_TO_OPEN_USERID`
        // `{"userid_list":[...]}`
        let list: Vec<serde_json::Value> = userid_list
            .iter()
            .map(|v| serde_json::Value::from(*v))
            .collect();
        let body = serde_json::json!({ "userid_list": list }).to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(USERID_TO_OPEN_USERID), &body)
            .await?;
        WxCpUseridToOpenUseridResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn open_userid_to_userid(
        &self,
        open_userid_list: &[&str],
        source_agent_id: &str,
    ) -> Result<WxCpOpenUseridToUseridResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `openUseridToUserid`：POST `OPEN_USERID_TO_USERID`
        // `{"open_userid_list":[...], "source_agentid":...}`
        let list: Vec<serde_json::Value> = open_userid_list
            .iter()
            .map(|v| serde_json::Value::from(*v))
            .collect();
        let body = serde_json::json!({
            "open_userid_list": list,
            "source_agentid": source_agent_id
        })
        .to_string();
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(&config.api_url(OPEN_USERID_TO_USERID), &body)
            .await?;
        WxCpOpenUseridToUseridResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn get_user_list_id(
        &self,
        cursor: &str,
        limit: Option<i32>,
    ) -> Result<WxCpDeptUserResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserListId`：POST `USER_LIST_ID`，`cursor`/`limit` 按非空条件
        let mut body = serde_json::Map::new();
        if !cursor.is_empty() {
            body.insert("cursor".to_string(), serde_json::Value::from(cursor));
        }
        if let Some(limit) = limit {
            body.insert("limit".to_string(), serde_json::Value::from(limit));
        }
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(
                &config.api_url(USER_LIST_ID),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        WxCpDeptUserResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}

/// 拼接部门成员查询参数（对应 Java `listByDepartment` 内 params 拼接：
/// `&fetch_child=1/0`（可选）、`&status=X`（缺省 0））。
fn build_department_params(fetch_child: Option<bool>, status: Option<i32>) -> String {
    let mut params = String::new();
    if let Some(fetch_child) = fetch_child {
        params.push_str(if fetch_child {
            "&fetch_child=1"
        } else {
            "&fetch_child=0"
        });
    }
    if let Some(status) = status {
        params.push_str(&format!("&status={status}"));
    } else {
        params.push_str("&status=0");
    }
    params
}

/// 解析响应 `userlist` 数组（对应 Java
/// `WxCpGsonBuilder.fromJson(jsonObject.get("userlist"), List<WxCpUser>)`）。
fn parse_user_list(response_content: &str) -> Result<Vec<WxCpUser>, WxErrorException> {
    let json: serde_json::Value = serde_json::from_str(response_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
    let list = json
        .get("userlist")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| WxErrorException::from_code(-99, "userlist 字段缺失"))?;
    list.iter()
        .map(|v| {
            serde_json::from_value(v.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
        })
        .collect()
}

/// 解析响应中的字符串字段（对应 Java `tmpJson.get(field).getAsString()`）。
fn parse_string_field(response_content: &str, field: &str) -> Result<String, WxErrorException> {
    let json: serde_json::Value = serde_json::from_str(response_content)
        .map_err(|e| WxErrorException::Serde(e.to_string()))?;
    json.get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| WxErrorException::from_code(-99, format!("{field} 字段缺失")))
}
