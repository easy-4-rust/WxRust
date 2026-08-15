//! 云开发服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaCloudServiceImpl`：
//! 云函数触发、云数据库（增删改查/聚合/索引/迁移）、云存储、腾讯云凭证、
//! 短信发送。请求体字段与查询语句拼接（Java `Joiner.on("").skipNulls()`
//! 的 `db.collection('X')...` 链）逐字对齐。

use std::collections::HashMap;
use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaCloudService;
use crate::bean::cloud::{
    WxCloudBatchDeleteFileResult, WxCloudBatchDownloadFileResult,
    WxCloudCloudDatabaseMigrateQueryInfoResult, WxCloudDatabaseCollectionGetResult,
    WxCloudDatabaseCreateIndexRequest, WxCloudDatabaseQueryResult, WxCloudDatabaseUpdateResult,
    WxCloudGetQcloudTokenResult, WxCloudSendSmsV2Request, WxCloudSendSmsV2Result,
    WxCloudUploadFileResult,
};
use crate::enums::g4_urls::url_g4_ability::cloud as cloud_url;

/// 云开发服务实现。
pub struct WxMaCloudServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaCloudServiceImpl {
    /// 构建云开发服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 从配置读取默认云环境 ID（对应 Java
    /// `wxMaService.getWxMaConfig().getCloudEnv()`）。
    fn cloud_env(svc: &dyn WxMaService) -> String {
        svc.wx_ma_config()
            .cloud_env()
            .unwrap_or_default()
            .to_string()
    }

    /// 发送 POST 请求并解析 JSON（对应 Java `post` + `GsonParser.parse`）。
    async fn post_json(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<serde_json::Value, WxErrorException> {
        let response = svc.post(url, post_body).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }

    /// 发送 POST 请求并解析为指定类型（对应 Java `post` + gson `fromJson`）。
    async fn post_as<T>(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_body).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaCloudService for WxMaCloudServiceImpl {
    /// 触发云函数（使用配置的默认云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.invokeCloudFunction(String, String)`）。
    async fn invoke_cloud_function(
        &self,
        name: &str,
        body: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.invoke_cloud_function_with_env(&cloud_env, name, body)
            .await
    }

    /// 触发云函数（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.invokeCloudFunction(String, String, String)`）。
    ///
    /// 注意：Java 实现**忽略** `env` 入参，URL 中的 env 始终取配置的
    /// `cloudEnv`（Java 原样行为，此处照搬并在注释标注）。
    async fn invoke_cloud_function_with_env(
        &self,
        env: &str,
        name: &str,
        body: &str,
    ) -> Result<String, WxErrorException> {
        let _ = env;
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let cloud_env = Self::cloud_env(svc.as_ref());
        let url = cloud_url::invoke_cloud_function_url(config.as_ref(), &cloud_env, name);
        let json = Self::post_json(svc.as_ref(), &url, body).await?;
        json.get("resp_data")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "resp_data 字段缺失"))
    }

    /// 批量添加记录到集合（对应 Java `WxMaCloudServiceImpl.add(String, List<?>)`）。
    async fn add(
        &self,
        collection: &str,
        list: &[serde_json::Value],
    ) -> Result<Vec<String>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let json_data = serde_json::Value::Array(list.to_vec()).to_string();
        let query = format!("db.collection('{collection}').add({{data: {json_data}}})");
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_add_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        let id_list = json
            .get("id_list")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let ids = id_list
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
        Ok(ids)
    }

    /// 添加单条记录到集合（对应 Java `WxMaCloudServiceImpl.add(String, Object)`）。
    async fn add_single(
        &self,
        collection: &str,
        obj: &serde_json::Value,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let json_data = obj.to_string();
        let query = format!("db.collection('{collection}').add({{data: {json_data}}})");
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_add_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        json.get("id_list")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "id_list 字段缺失"))
    }

    /// 数据库插入记录（对应 Java `WxMaCloudServiceImpl.databaseAdd(String)`）。
    async fn database_add(&self, query: &str) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_add_with_env(&cloud_env, query).await
    }

    /// 数据库插入记录（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseAdd(String, String)`）。
    async fn database_add_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_add_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        json.get("id_list")
            .cloned()
            .ok_or_else(|| WxErrorException::from_code(-99, "id_list 字段缺失"))
    }

    /// 删除集合中符合条件的记录（对应 Java
    /// `WxMaCloudServiceImpl.delete(String, String)`）。
    async fn delete(&self, collection: &str, where_json: &str) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let query = format!("db.collection('{collection}').where({where_json}).remove()");
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_delete_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("deleted").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
    }

    /// 数据库删除记录（对应 Java `WxMaCloudServiceImpl.databaseDelete(String)`）。
    async fn database_delete(&self, query: &str) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_delete_with_env(&cloud_env, query).await
    }

    /// 数据库删除记录（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseDelete(String, String)`）。
    async fn database_delete_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_delete_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("deleted").and_then(|v| v.as_i64()).unwrap_or(0) as i32)
    }

    /// 更新集合中符合条件的记录（对应 Java
    /// `WxMaCloudServiceImpl.update(String, String, String)`）。
    async fn update(
        &self,
        collection: &str,
        where_json: &str,
        update_json: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let query = format!(
            "db.collection('{collection}').where({where_json}).update({{data: {update_json} }})"
        );
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_update_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 数据库更新记录（对应 Java `WxMaCloudServiceImpl.databaseUpdate(String)`）。
    async fn database_update(
        &self,
        query: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_update_with_env(&cloud_env, query).await
    }

    /// 数据库更新记录（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseUpdate(String, String)`）。
    async fn database_update_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<WxCloudDatabaseUpdateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_update_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 查询集合中的记录（对应 Java
    /// `WxMaCloudServiceImpl.query(String, String, Map, Integer, Integer)`）。
    async fn query(
        &self,
        collection: &str,
        where_json: &str,
        order_by: Option<&HashMap<String, String>>,
        skip: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `StringUtils.isBlank(whereJson) ? "{}" : whereJson`
        let where_json = if where_json.trim().is_empty() {
            "{}".to_string()
        } else {
            where_json.to_string()
        };
        let mut order_by_sb = String::new();
        if let Some(order_by) = order_by {
            for (key, value) in order_by {
                order_by_sb.push_str(&format!(".orderBy('{key}', '{value}')"));
            }
        }
        // Java：limit 默认 100，skip 默认 0
        let limit = limit.unwrap_or(100);
        let skip = skip.unwrap_or(0);
        let query = format!(
            "db.collection('{collection}').where({where_json}){order_by_sb}.skip({skip}).limit({limit}).get()"
        );
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_query_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 数据库查询记录（对应 Java `WxMaCloudServiceImpl.databaseQuery(String)`）。
    async fn database_query(
        &self,
        query: &str,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_query_with_env(&cloud_env, query).await
    }

    /// 数据库查询记录（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseQuery(String, String)`）。
    async fn database_query_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<WxCloudDatabaseQueryResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_query_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 数据库聚合记录（对应 Java `WxMaCloudServiceImpl.databaseAggregate(String)`）。
    async fn database_aggregate(&self, query: &str) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_aggregate_with_env(&cloud_env, query).await
    }

    /// 数据库聚合记录（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseAggregate(String, String)`）。
    async fn database_aggregate_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_aggregate_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        json.get("data")
            .cloned()
            .ok_or_else(|| WxErrorException::from_code(-99, "data 字段缺失"))
    }

    /// 统计集合中符合条件的记录数（对应 Java
    /// `WxMaCloudServiceImpl.count(String, String)`）。
    async fn count(&self, collection: &str, where_json: &str) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let query = format!("db.collection('{collection}').where({where_json}).count()");
        let params = serde_json::json!({
            "env": Self::cloud_env(svc.as_ref()),
            "query": query,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_count_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("count").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    /// 统计集合记录数（对应 Java `WxMaCloudServiceImpl.databaseCount(String)`）。
    async fn database_count(&self, query: &str) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_count_with_env(&cloud_env, query).await
    }

    /// 统计集合记录数（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseCount(String, String)`）。
    async fn database_count_with_env(
        &self,
        env: &str,
        query: &str,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let params = serde_json::json!({ "env": env, "query": query });
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_count_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("count").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    /// 变更数据库索引（对应 Java
    /// `WxMaCloudServiceImpl.updateIndex(String, List, List)`）。
    async fn update_index(
        &self,
        collection_name: &str,
        create_indexes: &[WxCloudDatabaseCreateIndexRequest],
        drop_index_names: &[String],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.update_index_with_env(
            &cloud_env,
            collection_name,
            create_indexes,
            drop_index_names,
        )
        .await
    }

    /// 变更数据库索引（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.updateIndex(String, String, List, List)`）。
    ///
    /// `drop_index_names` 包装为 `[{"name": ...}]`（Java
    /// `ImmutableMap.of("name", index)`）。
    async fn update_index_with_env(
        &self,
        env: &str,
        collection_name: &str,
        create_indexes: &[WxCloudDatabaseCreateIndexRequest],
        drop_index_names: &[String],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let drop_indexes: Vec<serde_json::Value> = drop_index_names
            .iter()
            .map(|index| serde_json::json!({ "name": index }))
            .collect();
        let params = serde_json::json!({
            "env": env,
            "collection_name": collection_name,
            "create_indexes": create_indexes,
            "drop_indexes": drop_indexes,
        });
        let config = svc.wx_ma_config();
        svc.post(
            &cloud_url::update_index_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(())
    }

    /// 数据库导入（对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateImport(String, String, int, boolean, int)`）。
    async fn database_migrate_import(
        &self,
        collection_name: &str,
        file_path: &str,
        file_type: i32,
        stop_on_error: bool,
        conflict_mode: i32,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_migrate_import_with_env(
            &cloud_env,
            collection_name,
            file_path,
            file_type,
            stop_on_error,
            conflict_mode,
        )
        .await
    }

    /// 数据库导入（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateImport(String, String, String, int, boolean, int)`）。
    async fn database_migrate_import_with_env(
        &self,
        env: &str,
        collection_name: &str,
        file_path: &str,
        file_type: i32,
        stop_on_error: bool,
        conflict_mode: i32,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({
            "env": env,
            "collection_name": collection_name,
            "file_path": file_path,
            "file_type": file_type,
            "stop_on_error": stop_on_error,
            "conflict_mode": conflict_mode,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_migrate_import_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("job_id").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    /// 数据库导出（对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateExport(String, int, String)`）。
    async fn database_migrate_export(
        &self,
        file_path: &str,
        file_type: i32,
        query: &str,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_migrate_export_with_env(&cloud_env, file_path, file_type, query)
            .await
    }

    /// 数据库导出（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateExport(String, String, int, String)`）。
    async fn database_migrate_export_with_env(
        &self,
        env: &str,
        file_path: &str,
        file_type: i32,
        query: &str,
    ) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({
            "env": env,
            "file_path": file_path,
            "file_type": file_type,
            "query": query,
        });
        let config = svc.wx_ma_config();
        let json = Self::post_json(
            svc.as_ref(),
            &cloud_url::database_migrate_export_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(json.get("job_id").and_then(|v| v.as_i64()).unwrap_or(0))
    }

    /// 数据库迁移状态查询（对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateQueryInfo(Long)`）。
    async fn database_migrate_query_info(
        &self,
        job_id: i64,
    ) -> Result<WxCloudCloudDatabaseMigrateQueryInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_migrate_query_info_with_env(&cloud_env, job_id)
            .await
    }

    /// 数据库迁移状态查询（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseMigrateQueryInfo(String, Long)`）。
    async fn database_migrate_query_info_with_env(
        &self,
        env: &str,
        job_id: i64,
    ) -> Result<WxCloudCloudDatabaseMigrateQueryInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "env": env, "job_id": job_id });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_migrate_query_info_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 获取文件上传链接（对应 Java `WxMaCloudServiceImpl.uploadFile(String)`）。
    async fn upload_file(&self, path: &str) -> Result<WxCloudUploadFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.upload_file_with_env(&cloud_env, path).await
    }

    /// 获取文件上传链接（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.uploadFile(String, String)`）。
    async fn upload_file_with_env(
        &self,
        env: &str,
        path: &str,
    ) -> Result<WxCloudUploadFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "env": env, "path": path });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::upload_file_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 获取文件下载链接（对应 Java
    /// `WxMaCloudServiceImpl.batchDownloadFile(String[], long[])`）。
    async fn batch_download_file(
        &self,
        file_ids: &[String],
        max_ages: &[i64],
    ) -> Result<WxCloudBatchDownloadFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.batch_download_file_with_env(&cloud_env, file_ids, max_ages)
            .await
    }

    /// 获取文件下载链接（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.batchDownloadFile(String, String[], long[])`）。
    ///
    /// `file_list` 按 Java `ImmutableMap.of("fileid", fileId, "max_age", ...)`
    /// 组装。
    async fn batch_download_file_with_env(
        &self,
        env: &str,
        file_ids: &[String],
        max_ages: &[i64],
    ) -> Result<WxCloudBatchDownloadFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut file_list = Vec::new();
        for (i, file_id) in file_ids.iter().enumerate() {
            let max_age = max_ages.get(i).copied().unwrap_or(0);
            file_list.push(serde_json::json!({ "fileid": file_id, "max_age": max_age }));
        }
        let params = serde_json::json!({ "env": env, "file_list": file_list });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::batch_download_file_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 删除文件（对应 Java `WxMaCloudServiceImpl.batchDeleteFile(String[])`）。
    async fn batch_delete_file(
        &self,
        file_ids: &[String],
    ) -> Result<WxCloudBatchDeleteFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.batch_delete_file_with_env(&cloud_env, file_ids).await
    }

    /// 删除文件（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.batchDeleteFile(String, String[])`）。
    async fn batch_delete_file_with_env(
        &self,
        env: &str,
        file_ids: &[String],
    ) -> Result<WxCloudBatchDeleteFileResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "env": env, "fileid_list": file_ids });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::batch_delete_file_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 获取腾讯云 API 调用凭证（对应 Java
    /// `WxMaCloudServiceImpl.getQcloudToken(long)`）。
    async fn get_qcloud_token(
        &self,
        life_span: i64,
    ) -> Result<WxCloudGetQcloudTokenResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "lifespan": life_span });
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::get_qcloud_token_url(config.as_ref()),
            &params.to_string(),
        )
        .await
    }

    /// 新增集合（对应 Java `WxMaCloudServiceImpl.databaseCollectionAdd(String)`）。
    async fn database_collection_add(&self, collection_name: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_collection_add_with_env(&cloud_env, collection_name)
            .await
    }

    /// 新增集合（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseCollectionAdd(String, String)`）。
    async fn database_collection_add_with_env(
        &self,
        env: &str,
        collection_name: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "env": env, "collection_name": collection_name });
        let config = svc.wx_ma_config();
        svc.post(
            &cloud_url::database_collection_add_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(())
    }

    /// 删除集合（对应 Java `WxMaCloudServiceImpl.databaseCollectionDelete(String)`）。
    async fn database_collection_delete(
        &self,
        collection_name: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_collection_delete_with_env(&cloud_env, collection_name)
            .await
    }

    /// 删除集合（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseCollectionDelete(String, String)`）。
    async fn database_collection_delete_with_env(
        &self,
        env: &str,
        collection_name: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let params = serde_json::json!({ "env": env, "collection_name": collection_name });
        let config = svc.wx_ma_config();
        svc.post(
            &cloud_url::database_collection_delete_url(config.as_ref()),
            &params.to_string(),
        )
        .await?;
        Ok(())
    }

    /// 获取特定云环境下集合信息（对应 Java
    /// `WxMaCloudServiceImpl.databaseCollectionGet(Long, Long)`）。
    async fn database_collection_get(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<WxCloudDatabaseCollectionGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let cloud_env = Self::cloud_env(svc.as_ref());
        self.database_collection_get_with_env(&cloud_env, limit, offset)
            .await
    }

    /// 获取特定云环境下集合信息（指定云环境 ID，对应 Java
    /// `WxMaCloudServiceImpl.databaseCollectionGet(String, Long, Long)`；
    /// limit/offset 为 null 时不携带）。
    async fn database_collection_get_with_env(
        &self,
        env: &str,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<WxCloudDatabaseCollectionGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut params = serde_json::Map::new();
        params.insert("env".to_string(), serde_json::json!(env));
        if let Some(limit) = limit {
            params.insert("limit".to_string(), serde_json::json!(limit));
        }
        if let Some(offset) = offset {
            params.insert("offset".to_string(), serde_json::json!(offset));
        }
        let config = svc.wx_ma_config();
        Self::post_as(
            svc.as_ref(),
            &cloud_url::database_collection_get_url(config.as_ref()),
            &serde_json::Value::Object(params).to_string(),
        )
        .await
    }

    /// 发送携带 URL Link 的短信（对应 Java
    /// `WxMaCloudServiceImpl.sendSmsV2(WxCloudSendSmsV2Request)`）。
    ///
    /// request 未指定 env 时补默认云环境 ID（Java `request.getEnv() == null`
    /// 判断；Rust String 以空串表达未设置）。
    async fn send_sms_v2(
        &self,
        request: &WxCloudSendSmsV2Request,
    ) -> Result<WxCloudSendSmsV2Result, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut request = request.clone();
        if request.env.is_empty() {
            request.env = Self::cloud_env(svc.as_ref());
        }
        let config = svc.wx_ma_config();
        let post_body = serde_json::to_string(&request).map_err(WxErrorException::from)?;
        Self::post_as(
            svc.as_ref(),
            &cloud_url::send_sms_v2_url(config.as_ref()),
            &post_body,
        )
        .await
    }
}
