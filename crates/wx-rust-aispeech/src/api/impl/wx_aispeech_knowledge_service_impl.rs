//! 知识库助理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.impl.WxAispeechKnowledgeServiceImpl`：
//! 通过门面执行引擎（`executeKnowledgeGet/Post/Put/Delete/MultipartPost`）
//! 调用知识库 API，路径与查询参数严格对照 Java。

use std::collections::HashMap;
use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxAispeechKnowledgeService, WxAispeechService};
use crate::bean::knowledge::{
    KnowledgeInfo, KnowledgeListResult, KnowledgeManualCreateRequest, KnowledgeMoveProgress,
    KnowledgeMoveRequest, KnowledgeTagRequest, KnowledgeUpdateRequest, KnowledgeUrlCreateRequest,
};

/// 知识库助理服务实现。
pub struct WxAispeechKnowledgeServiceImpl {
    /// 门面服务弱引用（对应 Java `WxAispeechServiceImpl service` 字段）
    service: Weak<dyn WxAispeechService>,
}

impl WxAispeechKnowledgeServiceImpl {
    /// 构建实现。
    ///
    /// # 参数
    /// - `service`：门面服务弱引用（打破循环引用）
    pub fn new(service: Weak<dyn WxAispeechService>) -> Self {
        Self { service }
    }

    /// 门面服务引用（子服务生命周期内必然存在，对应 Java 强引用字段）。
    fn service(&self) -> Result<std::sync::Arc<dyn WxAispeechService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已释放"))
    }

    /// 解析知识信息列表（对应 Java `parseKnowledgeInfoList`）。
    ///
    /// 空响应 → `None`；响应为 JSON 对象且含 `data` 字段时解析 `data`，
    /// 否则按整个响应解析为数组。
    fn parse_knowledge_info_list(
        response: &str,
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException> {
        if response.is_empty() {
            return Ok(None);
        }
        let element: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = if let serde_json::Value::Object(object) = &element {
            if let Some(data) = object.get("data") {
                serde_json::from_value(data.clone())
            } else {
                serde_json::from_value(element.clone())
            }
        } else {
            serde_json::from_value(element)
        };
        list.map(Some)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxAispeechKnowledgeService for WxAispeechKnowledgeServiceImpl {
    async fn create_knowledge_by_file(
        &self,
        knowledge_base_id: &str,
        file_name: &str,
        file_bytes: &[u8],
        title: Option<&str>,
        description: Option<&str>,
        metadata: Option<&str>,
    ) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/knowledge/file");
        let response = service
            .execute_knowledge_multipart_post(
                &path,
                file_name,
                file_bytes,
                title,
                description,
                metadata,
            )
            .await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_knowledge_by_url(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeUrlCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/knowledge/url");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_post(&path, Some(&body)).await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_knowledge_by_manual(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeManualCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/knowledge/manual");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_post(&path, Some(&body)).await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn list_knowledge(
        &self,
        knowledge_base_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException> {
        // 对应 Java：page/page_size 为 null 时仍放入 query map（值为 null 由
        // 执行器跳过）
        let mut query = HashMap::new();
        query.insert(
            "page".to_string(),
            page.map(|v| v.to_string()).unwrap_or_default(),
        );
        query.insert(
            "page_size".to_string(),
            page_size.map(|v| v.to_string()).unwrap_or_default(),
        );
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/knowledge");
        let response = service.execute_knowledge_get(&path, Some(&query)).await?;
        let result: KnowledgeListResult =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // 对应 Java：result == null 时返回 null
        Ok(result.data)
    }

    async fn list_knowledge_by_ids(
        &self,
        knowledge_ids: &[String],
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException> {
        // 对应 Java：空列表直接返回 null
        if knowledge_ids.is_empty() {
            return Ok(None);
        }
        // 对应 Java StringJoiner：跳过空白 id；拼完后仍为空则返回 null
        let ids: Vec<&str> = knowledge_ids
            .iter()
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .collect();
        if ids.is_empty() {
            return Ok(None);
        }
        let mut query = HashMap::new();
        query.insert("ids".to_string(), ids.join(","));
        let service = self.service()?;
        let response = service
            .execute_knowledge_get("/api/v1/knowledge/batch", Some(&query))
            .await?;
        Self::parse_knowledge_info_list(&response)
    }

    async fn get_knowledge(&self, knowledge_id: &str) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge/{knowledge_id}");
        let response = service.execute_knowledge_get(&path, None).await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_knowledge(
        &self,
        knowledge_id: &str,
        request: &KnowledgeUpdateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge/{knowledge_id}");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_put(&path, Some(&body)).await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_manual_knowledge(
        &self,
        knowledge_id: &str,
        request: &KnowledgeManualCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge/manual/{knowledge_id}");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_put(&path, Some(&body)).await?;
        KnowledgeInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_knowledge(&self, knowledge_id: &str) -> Result<bool, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge/{knowledge_id}");
        service.execute_knowledge_delete(&path).await?;
        // 对应 Java：无异常即返回 true
        Ok(true)
    }

    async fn update_knowledge_tags(
        &self,
        knowledge_ids: &[String],
        tag_id: Option<i64>,
    ) -> Result<bool, WxErrorException> {
        // 对应 Java：空列表或 tagId 为 null 短路返回 false
        if knowledge_ids.is_empty() || tag_id.is_none() {
            return Ok(false);
        }
        let service = self.service()?;
        let body = serde_json::json!({
            "knowledge_ids": knowledge_ids,
            "tag_id": tag_id,
        })
        .to_string();
        let response = service
            .execute_knowledge_put("/api/v1/knowledge/tags", Some(&body))
            .await?;
        // 对应 Java：响应非空为 true
        Ok(!response.is_empty())
    }

    async fn search_knowledge(
        &self,
        keyword: &str,
        knowledge_base_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException> {
        // 对应 Java：keyword/knowledge_base_id 原样放入 query（null 由执行器跳过）
        let mut query = HashMap::new();
        query.insert("keyword".to_string(), keyword.to_string());
        query.insert(
            "knowledge_base_id".to_string(),
            knowledge_base_id.to_string(),
        );
        query.insert(
            "page".to_string(),
            page.map(|v| v.to_string()).unwrap_or_default(),
        );
        query.insert(
            "page_size".to_string(),
            page_size.map(|v| v.to_string()).unwrap_or_default(),
        );
        let service = self.service()?;
        let response = service
            .execute_knowledge_get("/api/v1/knowledge/search", Some(&query))
            .await?;
        Self::parse_knowledge_info_list(&response)
    }

    async fn move_knowledge(
        &self,
        request: &KnowledgeMoveRequest,
    ) -> Result<String, WxErrorException> {
        let service = self.service()?;
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        service
            .execute_knowledge_post("/api/v1/knowledge/move", Some(&body))
            .await
    }

    async fn get_move_progress(
        &self,
        task_id: &str,
    ) -> Result<KnowledgeMoveProgress, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge/move/progress/{task_id}");
        let response = service.execute_knowledge_get(&path, None).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn create_knowledge_base_tag(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeTagRequest,
    ) -> Result<bool, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/tags");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_post(&path, Some(&body)).await?;
        Ok(!response.is_empty())
    }

    async fn update_knowledge_base_tag(
        &self,
        knowledge_base_id: &str,
        tag_id: &str,
        request: &KnowledgeTagRequest,
    ) -> Result<bool, WxErrorException> {
        let service = self.service()?;
        let path = format!("/api/v1/knowledge-bases/{knowledge_base_id}/tags/{tag_id}");
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = service.execute_knowledge_put(&path, Some(&body)).await?;
        Ok(!response.is_empty())
    }

    async fn post_raw(
        &self,
        path: &str,
        request_body: Option<&str>,
    ) -> Result<String, WxErrorException> {
        self.service()?
            .execute_knowledge_post(path, request_body)
            .await
    }

    async fn get_raw(
        &self,
        path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<String, WxErrorException> {
        self.service()?
            .execute_knowledge_get(path, query_params)
            .await
    }
}
