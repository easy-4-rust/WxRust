//! 企业微信第三方应用标签服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpTagServiceImpl`
//! （部分照搬 `WxCpTagServiceImpl`）：以 `Weak<dyn WxCpTpService>` 持有
//! 门面。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{WxCpTpTag, WxCpTpTagAddOrRemoveUsersResult, WxCpTpTagGetResult};
use crate::enums::url_tag;
use crate::tp::service::{WxCpTpService, WxCpTpTagService};

/// 企业微信第三方应用标签服务实现。
pub struct WxCpTpTagServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpTagServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 以 JSON 创建标签（对应 Java `create(JsonObject)`）。
    async fn create_with_json(
        &self,
        param: &serde_json::Value,
    ) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tag::TAG_CREATE);
        let response_content = service.post(&url, &param.to_string()).await?;
        let json: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get("tagid")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .ok_or_else(|| WxErrorException::from_code(-99, "tagid 字段缺失"))
    }
}

#[async_trait]
impl WxCpTpTagService for WxCpTpTagServiceImpl {
    async fn create(&self, name: &str, id: Option<i32>) -> Result<String, WxErrorException> {
        let mut body = serde_json::Map::new();
        body.insert(
            "tagname".to_string(),
            serde_json::Value::String(name.to_string()),
        );
        if let Some(id) = id {
            body.insert("tagid".to_string(), serde_json::Value::Number(id.into()));
        }
        self.create_with_json(&serde_json::Value::Object(body))
            .await
    }

    async fn update(&self, tag_id: &str, tag_name: &str) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tag::TAG_UPDATE);
        let body = serde_json::json!({
            "tagid": tag_id,
            "tagname": tag_name,
        })
        .to_string();
        service.post(&url, &body).await?;
        Ok(())
    }

    async fn delete(&self, tag_id: &str) -> Result<(), WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java String.format(TAG_DELETE, tagId)
        let url = config.api_url(url_tag::TAG_DELETE).replace("%s", tag_id);
        service.get(&url, "").await?;
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<WxCpTpTag>, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tag::TAG_LIST);
        let response_content = service.get(&url, "").await?;
        let tmp: serde_json::Value = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = tmp
            .get("taglist")
            .cloned()
            .unwrap_or(serde_json::Value::Array(vec![]));
        serde_json::from_value(list)
            .map_err(|e| WxErrorException::Serde(format!("taglist 解析失败: {e}")))
    }

    async fn get(&self, tag_id: &str) -> Result<WxCpTpTagGetResult, WxErrorException> {
        if tag_id.trim().is_empty() {
            return Err(WxErrorException::from_code(-99, "缺少tagId参数"));
        }
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        // Java String.format(TAG_GET, tagId)
        let url = config.api_url(url_tag::TAG_GET).replace("%s", tag_id);
        let response_content = service.get(&url, "").await?;
        // Java WxCpTpTagGetResult.deserialize（= 父类 fromJson 语义）
        WxCpTpTagGetResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn add_users_2_tag(
        &self,
        tag_id: &str,
        user_ids: &[String],
        party_ids: &[String],
    ) -> Result<WxCpTpTagAddOrRemoveUsersResult, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tag::TAG_ADD_TAG_USERS);
        let body = build_tag_users_json(tag_id, user_ids, party_ids);
        let response = service.post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn remove_users_from_tag(
        &self,
        tag_id: &str,
        user_ids: &[String],
        party_ids: &[String],
    ) -> Result<WxCpTpTagAddOrRemoveUsersResult, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tag::TAG_DEL_TAG_USERS);
        let body = build_tag_users_json(tag_id, user_ids, party_ids);
        let response = service.post(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

/// 组装 tagid/userlist/partylist 请求体（对应 Java
/// `addUserIdsAndPartyIdsToJson`：列表为空时省略对应字段）。
fn build_tag_users_json(
    tag_id: &str,
    user_ids: &[String],
    party_ids: &[String],
) -> serde_json::Value {
    let mut body = serde_json::Map::new();
    body.insert(
        "tagid".to_string(),
        serde_json::Value::String(tag_id.to_string()),
    );
    if !user_ids.is_empty() {
        body.insert(
            "userlist".to_string(),
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
            "partylist".to_string(),
            serde_json::Value::Array(
                party_ids
                    .iter()
                    .map(|v| serde_json::Value::String(v.clone()))
                    .collect(),
            ),
        );
    }
    serde_json::Value::Object(body)
}
