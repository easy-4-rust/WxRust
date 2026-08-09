//! 知识库助理服务接口。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.WxAispeechKnowledgeService`。

use std::collections::HashMap;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::knowledge::{
    KnowledgeInfo, KnowledgeManualCreateRequest, KnowledgeMoveProgress, KnowledgeMoveRequest,
    KnowledgeTagRequest, KnowledgeUpdateRequest, KnowledgeUrlCreateRequest,
};

/// 知识库助理服务。
#[async_trait]
pub trait WxAispeechKnowledgeService: Send + Sync {
    /// 通过文件创建知识（对应 Java `createKnowledgeByFile`）。
    ///
    /// Java `File` 以字节 + 文件名承载（ADAPTED）。multipart/form-data
    /// 上传 `/api/v1/knowledge-bases/{kb}/knowledge/file`。
    async fn create_knowledge_by_file(
        &self,
        knowledge_base_id: &str,
        file_name: &str,
        file_bytes: &[u8],
        title: Option<&str>,
        description: Option<&str>,
        metadata: Option<&str>,
    ) -> Result<KnowledgeInfo, WxErrorException>;

    /// 通过 URL 创建知识（对应 Java `createKnowledgeByUrl`）。
    async fn create_knowledge_by_url(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeUrlCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException>;

    /// 手工创建知识（对应 Java `createKnowledgeByManual`）。
    async fn create_knowledge_by_manual(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeManualCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException>;

    /// 分页列出知识（对应 Java `listKnowledge`），响应无 `data` 时返回
    /// `None`（对应 Java 返回 null）。
    async fn list_knowledge(
        &self,
        knowledge_base_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException>;

    /// 按 id 批量查询知识（对应 Java `listKnowledgeByIds`）。
    ///
    /// 列表为空或全为空白 id 时返回 `None`（对应 Java 返回 null）；
    /// 非空 id 以逗号拼接为 `ids` 查询参数。
    async fn list_knowledge_by_ids(
        &self,
        knowledge_ids: &[String],
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException>;

    /// 查询单个知识（对应 Java `getKnowledge`）。
    async fn get_knowledge(&self, knowledge_id: &str) -> Result<KnowledgeInfo, WxErrorException>;

    /// 更新知识（对应 Java `updateKnowledge`，PUT）。
    async fn update_knowledge(
        &self,
        knowledge_id: &str,
        request: &KnowledgeUpdateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException>;

    /// 更新手工知识（对应 Java `updateManualKnowledge`，PUT）。
    async fn update_manual_knowledge(
        &self,
        knowledge_id: &str,
        request: &KnowledgeManualCreateRequest,
    ) -> Result<KnowledgeInfo, WxErrorException>;

    /// 删除知识（对应 Java `deleteKnowledge`，恒返回 true）。
    async fn delete_knowledge(&self, knowledge_id: &str) -> Result<bool, WxErrorException>;

    /// 批量更新知识标签（对应 Java `updateKnowledgeTags`）。
    ///
    /// `knowledge_ids` 为空或 `tag_id` 为 null 时返回 false（对应 Java
    /// 短路返回）。
    async fn update_knowledge_tags(
        &self,
        knowledge_ids: &[String],
        tag_id: Option<i64>,
    ) -> Result<bool, WxErrorException>;

    /// 检索知识（对应 Java `searchKnowledge`）。
    async fn search_knowledge(
        &self,
        keyword: &str,
        knowledge_base_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<Option<Vec<KnowledgeInfo>>, WxErrorException>;

    /// 迁移知识（对应 Java `moveKnowledge`），返回任务 id 响应体。
    async fn move_knowledge(
        &self,
        request: &KnowledgeMoveRequest,
    ) -> Result<String, WxErrorException>;

    /// 查询迁移进度（对应 Java `getMoveProgress`）。
    async fn get_move_progress(
        &self,
        task_id: &str,
    ) -> Result<KnowledgeMoveProgress, WxErrorException>;

    /// 创建知识库标签（对应 Java `createKnowledgeBaseTag`），响应非空为
    /// 成功。
    async fn create_knowledge_base_tag(
        &self,
        knowledge_base_id: &str,
        request: &KnowledgeTagRequest,
    ) -> Result<bool, WxErrorException>;

    /// 更新知识库标签（对应 Java `updateKnowledgeBaseTag`），响应非空为
    /// 成功。
    async fn update_knowledge_base_tag(
        &self,
        knowledge_base_id: &str,
        tag_id: &str,
        request: &KnowledgeTagRequest,
    ) -> Result<bool, WxErrorException>;

    /// 原始 POST（对应 Java `postRaw(String path, Object requestBody)`）。
    ///
    /// `request_body` 为调用方序列化后的 JSON 字符串；`None` 等价于
    /// `toBody(null)` 的 `"{}"`。
    async fn post_raw(
        &self,
        path: &str,
        request_body: Option<&str>,
    ) -> Result<String, WxErrorException>;

    /// 原始 GET（对应 Java `getRaw(String path, Map<String, String>
    /// queryParams)`）。
    async fn get_raw(
        &self,
        path: &str,
        query_params: Option<&HashMap<String, String>>,
    ) -> Result<String, WxErrorException>;
}
