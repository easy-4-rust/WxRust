use crate::api::WxCpService;
use crate::bean::WxCpBaseResp;
use crate::bean::kf::{
    WxCpKfKnowledgeGroup, WxCpKfKnowledgeGroupAddResp, WxCpKfKnowledgeGroupListResp,
    WxCpKfKnowledgeIntent, WxCpKfKnowledgeIntentAddResp, WxCpKfKnowledgeIntentListResp,
};
use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

/// 客服知识库扩展，复用企业微信服务配置和认证，不改变旧客服 trait 的必需方法。
/// 对应 Java: me.chanjar.weixin.cp.api.WxCpKfService（知识库方法）
#[async_trait]
pub trait WxCpKfKnowledgeService: WxCpService {
    /// add Group：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#addKnowledgeGroup
    async fn add_knowledge_group(
        &self,
        group: &WxCpKfKnowledgeGroup,
    ) -> Result<WxCpKfKnowledgeGroupAddResp, WxErrorException> {
        let body =
            serde_json::to_string(group).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/add_group");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// del Group：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#delKnowledgeGroup
    async fn del_knowledge_group(&self, group_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let body = serde_json::json!({"group_id": group_id}).to_string();
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/del_group");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// mod Group：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#modKnowledgeGroup
    async fn mod_knowledge_group(
        &self,
        group: &WxCpKfKnowledgeGroup,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let body =
            serde_json::to_string(group).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/mod_group");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// list Group：接收可选游标、页大小与筛选条件，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#listKnowledgeGroup
    async fn list_knowledge_group(
        &self,
        cursor: Option<&str>,
        limit: Option<i32>,
        group_id: Option<&str>,
    ) -> Result<WxCpKfKnowledgeGroupListResp, WxErrorException> {
        let mut body = serde_json::Map::new();
        if let Some(v) = cursor {
            body.insert("cursor".into(), v.into());
        }
        if let Some(v) = limit {
            body.insert("limit".into(), v.into());
        }
        if let Some(v) = group_id {
            body.insert("group_id".into(), v.into());
        }
        let body = serde_json::Value::Object(body).to_string();
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/list_group");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// add Intent：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#addKnowledgeIntent
    async fn add_knowledge_intent(
        &self,
        intent: &WxCpKfKnowledgeIntent,
    ) -> Result<WxCpKfKnowledgeIntentAddResp, WxErrorException> {
        let body =
            serde_json::to_string(intent).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/add_intent");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// del Intent：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#delKnowledgeIntent
    async fn del_knowledge_intent(
        &self,
        intent_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let body = serde_json::json!({"intent_id": intent_id}).to_string();
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/del_intent");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// mod Intent：接收分组或问答参数，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#modKnowledgeIntent
    async fn mod_knowledge_intent(
        &self,
        intent: &WxCpKfKnowledgeIntent,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let body =
            serde_json::to_string(intent).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/mod_intent");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// list Intent：接收可选游标、页大小与筛选条件，返回业务响应或请求错误。
    /// 对应 Java: WxCpKfService#listKnowledgeIntent
    async fn list_knowledge_intent(
        &self,
        cursor: Option<&str>,
        limit: Option<i32>,
        group_id: Option<&str>,
        intent_id: Option<&str>,
    ) -> Result<WxCpKfKnowledgeIntentListResp, WxErrorException> {
        let mut body = serde_json::Map::new();
        if let Some(v) = cursor {
            body.insert("cursor".into(), v.into());
        }
        if let Some(v) = limit {
            body.insert("limit".into(), v.into());
        }
        if let Some(v) = group_id {
            body.insert("group_id".into(), v.into());
        }
        if let Some(v) = intent_id {
            body.insert("intent_id".into(), v.into());
        }
        let body = serde_json::Value::Object(body).to_string();
        let url = self
            .wx_cp_config_storage()
            .api_url("/cgi-bin/kf/knowledge/list_intent");
        let response = self.post(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

impl<T: WxCpService + ?Sized> WxCpKfKnowledgeService for T {}
