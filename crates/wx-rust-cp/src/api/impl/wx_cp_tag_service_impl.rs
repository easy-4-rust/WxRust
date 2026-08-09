//! 标签服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpTagServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpService, WxCpTagService};
use crate::bean::{WxCpTag, WxCpTagAddOrRemoveUsersResult, WxCpTagGetResult, WxCpUser};
use crate::enums::url_tag::*;

/// 标签服务实现。
pub struct WxCpTagServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpTagServiceImpl {
    /// 构建标签服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxCpTagService for WxCpTagServiceImpl {
    async fn create(&self, name: &str, id: Option<i32>) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `create`：`{"tagname":..., "tagid":...}`（tagid 非必填），
        // POST `TAG_CREATE`，响应取 `tagid`
        let mut body = serde_json::Map::new();
        body.insert("tagname".to_string(), serde_json::Value::from(name));
        if let Some(id) = id {
            body.insert("tagid".to_string(), serde_json::Value::from(id));
        }
        let config = svc.wx_cp_config_storage();
        let response_content = svc
            .post(
                &config.api_url(TAG_CREATE),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("tagid")
            .and_then(serde_json::Value::as_str)
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "tagid 字段缺失"))
    }

    async fn update(&self, tag_id: &str, tag_name: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `update`：POST `TAG_UPDATE` `{"tagid":..., "tagname":...}`
        let body = serde_json::json!({ "tagid": tag_id, "tagname": tag_name }).to_string();
        let config = svc.wx_cp_config_storage();
        svc.post(&config.api_url(TAG_UPDATE), &body).await?;
        Ok(())
    }

    async fn delete(&self, tag_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `delete`：GET `String.format(TAG_DELETE, tagId)`（`%s` 替换）
        let config = svc.wx_cp_config_storage();
        let url = config.api_url(TAG_DELETE).replace("%s", tag_id);
        svc.get(&url, "").await?;
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<WxCpTag>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listAll`：GET `TAG_LIST`，响应取 `taglist` 数组
        let config = svc.wx_cp_config_storage();
        let response_content = svc.get(&config.api_url(TAG_LIST), "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("taglist")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| WxErrorException::from_code(-99, "taglist 字段缺失"))?;
        list.iter()
            .map(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| WxErrorException::Serde(e.to_string()))
            })
            .collect()
    }

    async fn list_users_by_tag_id(&self, tag_id: &str) -> Result<Vec<WxCpUser>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `listUsersByTagId`：GET `TAG_GET`，响应取 `userlist` 数组
        let config = svc.wx_cp_config_storage();
        let url = config.api_url(TAG_GET).replace("%s", tag_id);
        let response_content = svc.get(&url, "").await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("userlist")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| WxErrorException::from_code(-99, "userlist 字段缺失"))?;
        list.iter()
            .map(|v| {
                serde_json::from_value(v.clone())
                    .map_err(|e| WxErrorException::Serde(e.to_string()))
            })
            .collect()
    }

    async fn get(&self, tag_id: &str) -> Result<WxCpTagGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `get`：GET `TAG_GET`，整体响应 `WxCpTagGetResult.fromJson`
        let config = svc.wx_cp_config_storage();
        let url = config.api_url(TAG_GET).replace("%s", tag_id);
        let response_content = svc.get(&url, "").await?;
        WxCpTagGetResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn add_users2_tag(
        &self,
        tag_id: &str,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpTagAddOrRemoveUsersResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `addUsers2Tag`：POST `TAG_ADD_TAG_USERS`
        // `{"tagid":..., "userlist":[...]?, "partylist":[...]?}`
        let body = build_tag_users_body(tag_id, user_ids, party_ids);
        let config = svc.wx_cp_config_storage();
        let response_content = svc.post(&config.api_url(TAG_ADD_TAG_USERS), &body).await?;
        WxCpTagAddOrRemoveUsersResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn remove_users_from_tag(
        &self,
        tag_id: &str,
        user_ids: &[&str],
        party_ids: &[&str],
    ) -> Result<WxCpTagAddOrRemoveUsersResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `removeUsersFromTag`：POST `TAG_DEL_TAG_USERS`，请求体同构
        let body = build_tag_users_body(tag_id, user_ids, party_ids);
        let config = svc.wx_cp_config_storage();
        let response_content = svc.post(&config.api_url(TAG_DEL_TAG_USERS), &body).await?;
        WxCpTagAddOrRemoveUsersResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }
}

/// 组装标签成员请求体（对应 Java `addUserIdsAndPartyIdsToJson`：
/// `tagid` 恒有，`userlist`/`partylist` 按非 null 条件追加；Rust 以空切片
/// 判空，ADAPTED：无 null 语义）。
fn build_tag_users_body(tag_id: &str, user_ids: &[&str], party_ids: &[&str]) -> String {
    let mut body = serde_json::Map::new();
    body.insert("tagid".to_string(), serde_json::Value::from(tag_id));
    if !user_ids.is_empty() {
        body.insert(
            "userlist".to_string(),
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
            "partylist".to_string(),
            serde_json::Value::Array(
                party_ids
                    .iter()
                    .map(|v| serde_json::Value::from(*v))
                    .collect(),
            ),
        );
    }
    serde_json::Value::Object(body).to_string()
}
