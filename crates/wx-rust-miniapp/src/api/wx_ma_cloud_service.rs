//! 云开发服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaCloudService`
//! （`impl.WxMaCloudServiceImpl`）。

use std::collections::HashMap;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::cloud::{
    WxCloudBatchDeleteFileResult, WxCloudBatchDownloadFileResult,
    WxCloudCloudDatabaseMigrateQueryInfoResult, WxCloudDatabaseCollectionGetResult,
    WxCloudDatabaseCreateIndexRequest, WxCloudDatabaseQueryResult, WxCloudDatabaseUpdateResult,
    WxCloudGetQcloudTokenResult, WxCloudSendSmsV2Request, WxCloudSendSmsV2Result,
    WxCloudUploadFileResult,
};

/// 云开发服务。
///
/// 对应 Java `WxMaCloudService`：云函数触发、云数据库（增删改查/聚合/索引/
/// 迁移）、云存储（上传/下载/删除）、腾讯云 API 凭证、短信发送。
#[async_trait]
pub trait WxMaCloudService: Send + Sync {
    /// 触发云函数（使用配置的默认云环境 ID，对应 Java
    /// `invokeCloudFunction(String, String)`）。
    async fn invoke_cloud_function(
        &self,
        name: &str,
        body: &str,
    ) -> Result<String, WxErrorException>;

    /// 触发云函数（指定云环境 ID，对应 Java
    /// `invokeCloudFunction(String, String, String)`）。
    async fn invoke_cloud_function_with_env(
        &self,
        env: &str,
        name: &str,
        body: &str,
    ) -> Result<String, WxErrorException>;

    /// 批量添加记录到集合（对应 Java `add(String, List<?>)`）。
    async fn add(
        &self,
        collection: &str,
        list: &[serde_json::Value],
    ) -> Result<Vec<String>, WxErrorException>;

    /// 添加单条记录到集合（对应 Java `add(String, Object)`）。
    async fn add_single(
        &self,
        collection: &str,
        obj: &serde_json::Value,
    ) -> Result<String, WxErrorException>;

    /// 数据库插入记录（对应 Java `databaseAdd(String)`）。
    async fn database_add(&self, query: &str) -> Result<serde_json::Value, WxErrorException>;

    /// 数据库插入记录（指定云环境 ID，对应 Java `databaseAdd(String, String)`）。
    async fn database_add_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<serde_json::Value, WxErrorException>;

    /// 删除集合中符合条件的记录（对应 Java `delete(String, String)`）。
    async fn delete(&self, collection: &str, where_json: &str) -> Result<i32, WxErrorException>;

    /// 数据库删除记录（对应 Java `databaseDelete(String)`）。
    async fn database_delete(&self, query: &str) -> Result<i32, WxErrorException>;

    /// 数据库删除记录（指定云环境 ID，对应 Java `databaseDelete(String, String)`）。
    async fn database_delete_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<i32, WxErrorException>;

    /// 更新集合中符合条件的记录（对应 Java `update(String, String, String)`）。
    async fn update(
        &self,
        collection: &str,
        where_json: &str,
        update_json: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException>;

    /// 数据库更新记录（对应 Java `databaseUpdate(String)`）。
    async fn database_update(
        &self,
        query: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException>;

    /// 数据库更新记录（指定云环境 ID，对应 Java `databaseUpdate(String, String)`）。
    async fn database_update_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException>;

    /// 查询集合中的记录（对应 Java `query(String, String, Map, Integer, Integer)`）。
    async fn query(
        &self,
        collection: &str,
        where_json: &str,
        order_by: Option<&HashMap<String, String>>,
        skip: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException>;

    /// 数据库查询记录（对应 Java `databaseQuery(String)`）。
    async fn database_query(
        &self,
        query: &str,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException>;

    /// 数据库查询记录（指定云环境 ID，对应 Java `databaseQuery(String, String)`）。
    async fn database_query_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException>;

    /// 数据库聚合记录（对应 Java `databaseAggregate(String)`）。
    async fn database_aggregate(&self, query: &str) -> Result<serde_json::Value, WxErrorException>;

    /// 数据库聚合记录（指定云环境 ID，对应 Java `databaseAggregate(String, String)`）。
    async fn database_aggregate_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<serde_json::Value, WxErrorException>;

    /// 统计集合中符合条件的记录数（对应 Java `count(String, String)`）。
    async fn count(&self, collection: &str, where_json: &str) -> Result<i64, WxErrorException>;

    /// 统计集合记录数（对应 Java `databaseCount(String)`）。
    async fn database_count(&self, query: &str) -> Result<i64, WxErrorException>;

    /// 统计集合记录数（指定云环境 ID，对应 Java `databaseCount(String, String)`）。
    async fn database_count_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<i64, WxErrorException>;

    /// 变更数据库索引（对应 Java `updateIndex(String, List, List)`）。
    async fn update_index(
        &self,
        collection_name: &str,
        create_indexes: &[WxCloudDatabaseCreateIndexRequest],
        drop_index_names: &[String],
    ) -> Result<(), WxErrorException>;

    /// 变更数据库索引（指定云环境 ID，对应 Java
    /// `updateIndex(String, String, List, List)`）。
    async fn update_index_with_env(
        &self,
        env: &str,
        collection_name: &str,
        create_indexes: &[WxCloudDatabaseCreateIndexRequest],
        drop_index_names: &[String],
    ) -> Result<(), WxErrorException>;

    /// 数据库导入（对应 Java `databaseMigrateImport(String, String, int, boolean, int)`）。
    async fn database_migrate_import(
        &self,
        collection_name: &str,
        file_path: &str,
        file_type: i32,
        stop_on_error: bool,
        conflict_mode: i32,
    ) -> Result<i64, WxErrorException>;

    /// 数据库导入（指定云环境 ID，对应 Java
    /// `databaseMigrateImport(String, String, String, int, boolean, int)`）。
    async fn database_migrate_import_with_env(
        &self,
        env: &str,
        collection_name: &str,
        file_path: &str,
        file_type: i32,
        stop_on_error: bool,
        conflict_mode: i32,
    ) -> Result<i64, WxErrorException>;

    /// 数据库导出（对应 Java `databaseMigrateExport(String, int, String)`）。
    async fn database_migrate_export(
        &self,
        file_path: &str,
        file_type: i32,
        query: &str,
    ) -> Result<i64, WxErrorException>;

    /// 数据库导出（指定云环境 ID，对应 Java
    /// `databaseMigrateExport(String, String, int, String)`）。
    async fn database_migrate_export_with_env(
        &self,
        env: &str,
        file_path: &str,
        file_type: i32,
        query: &str,
    ) -> Result<i64, WxErrorException>;

    /// 数据库迁移状态查询（对应 Java `databaseMigrateQueryInfo(Long)`）。
    async fn database_migrate_query_info(
        &self,
        job_id: i64,
    ) -> Result<WxCloudCloudDatabaseMigrateQueryInfoResult, WxErrorException>;

    /// 数据库迁移状态查询（指定云环境 ID，对应 Java
    /// `databaseMigrateQueryInfo(String, Long)`）。
    async fn database_migrate_query_info_with_env(
        &self,
        env: &str,
        job_id: i64,
    ) -> Result<WxCloudCloudDatabaseMigrateQueryInfoResult, WxErrorException>;

    /// 获取文件上传链接（对应 Java `uploadFile(String)`）。
    async fn upload_file(&self, path: &str) -> Result<WxCloudUploadFileResult, WxErrorException>;

    /// 获取文件上传链接（指定云环境 ID，对应 Java `uploadFile(String, String)`）。
    async fn upload_file_with_env(
        &self,
        env: &str,
        path: &str,
    ) -> Result<WxCloudUploadFileResult, WxErrorException>;

    /// 获取文件下载链接（对应 Java `batchDownloadFile(String[], long[])`）。
    async fn batch_download_file(
        &self,
        file_ids: &[String],
        max_ages: &[i64],
    ) -> Result<WxCloudBatchDownloadFileResult, WxErrorException>;

    /// 获取文件下载链接（指定云环境 ID，对应 Java
    /// `batchDownloadFile(String, String[], long[])`）。
    async fn batch_download_file_with_env(
        &self,
        env: &str,
        file_ids: &[String],
        max_ages: &[i64],
    ) -> Result<WxCloudBatchDownloadFileResult, WxErrorException>;

    /// 删除文件（对应 Java `batchDeleteFile(String[])`）。
    async fn batch_delete_file(
        &self,
        file_ids: &[String],
    ) -> Result<WxCloudBatchDeleteFileResult, WxErrorException>;

    /// 删除文件（指定云环境 ID，对应 Java `batchDeleteFile(String, String[])`）。
    async fn batch_delete_file_with_env(
        &self,
        env: &str,
        file_ids: &[String],
    ) -> Result<WxCloudBatchDeleteFileResult, WxErrorException>;

    /// 获取腾讯云 API 调用凭证（对应 Java `getQcloudToken(long)`）。
    async fn get_qcloud_token(
        &self,
        life_span: i64,
    ) -> Result<WxCloudGetQcloudTokenResult, WxErrorException>;

    /// 新增集合（对应 Java `databaseCollectionAdd(String)`）。
    async fn database_collection_add(&self, collection_name: &str) -> Result<(), WxErrorException>;

    /// 新增集合（指定云环境 ID，对应 Java `databaseCollectionAdd(String, String)`）。
    async fn database_collection_add_with_env(
        &self,
        env: &str,
        collection_name: &str,
    ) -> Result<(), WxErrorException>;

    /// 删除集合（对应 Java `databaseCollectionDelete(String)`）。
    async fn database_collection_delete(
        &self,
        collection_name: &str,
    ) -> Result<(), WxErrorException>;

    /// 删除集合（指定云环境 ID，对应 Java `databaseCollectionDelete(String, String)`）。
    async fn database_collection_delete_with_env(
        &self,
        env: &str,
        collection_name: &str,
    ) -> Result<(), WxErrorException>;

    /// 获取特定云环境下集合信息（对应 Java `databaseCollectionGet(Long, Long)`）。
    async fn database_collection_get(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<WxCloudDatabaseCollectionGetResult, WxErrorException>;

    /// 获取特定云环境下集合信息（指定云环境 ID，对应 Java
    /// `databaseCollectionGet(String, Long, Long)`）。
    async fn database_collection_get_with_env(
        &self,
        env: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<WxCloudDatabaseCollectionGetResult, WxErrorException>;

    /// 发送携带 URL Link 的短信（对应 Java `sendSmsV2(WxCloudSendSmsV2Request)`）。
    async fn send_sms_v2(
        &self,
        request: &WxCloudSendSmsV2Request,
    ) -> Result<WxCloudSendSmsV2Result, WxErrorException>;
}
