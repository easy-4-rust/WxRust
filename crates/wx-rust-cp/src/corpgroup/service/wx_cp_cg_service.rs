//! 企业微信企业互联集团服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.corpgroup.service.WxCpCgService` 与
//! `BaseWxCpCgServiceImpl` 暴露的全部方法。Java 继承链
//! （`WxCpCgServiceApacheHttpClientImpl` → `BaseWxCpCgServiceImpl`）
//! 在 Rust 以 trait 默认实现 + 组合表达（与 `WxCpService` 门面同一设计
//! 原则）：
//! - 「Base」的 corp access token 双检锁刷新、通用执行通道
//!   （get/post/execute 带 corp access token）、小程序 session 转换等为
//!   本 trait 的默认实现（对应 Java `BaseWxCpCgServiceImpl`）；
//! - 执行引擎（指数退避重试 + 42009 自动单次刷新）抽为
//!   `corpgroup::service::r#impl::base_wx_cp_cg_service_impl` 的泛型
//!   自由函数（trait 无法携带泛型方法）；
//! - 配置存储/HTTP 客户端/`WxCpService` 引用/互联企业服务由具体实现
//!   `WxCpCgServiceImpl`（base_wx_cp_cg_service_impl.rs）提供；
//!   Java `getRequestHttp()` 以 `http_client()` 表达（reqwest 统一 HTTP，
//!   ADAPTED）。

use std::sync::Arc;

use async_trait::async_trait;

use wx_rust_common::bean::WxAccessToken;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::{SimpleGetRequestExecutor, SimplePostRequestExecutor};

use crate::api::WxCpService;
use crate::bean::{WxCpCorpGroupCorpGetTokenReq, WxCpMaTransferSession};
use crate::config::WxCpCorpGroupConfigStorage;
use crate::corpgroup::service::WxCpLinkedCorpService;
use crate::enums::url_corp_group;

/// 企业微信企业互联集团服务。
#[async_trait]
pub trait WxCpCgService: Send + Sync {
    // ---- 配置存储 / HTTP / WxCpService 引用（由具体实现提供） ----

    /// 获取配置存储（对应 Java `getWxCpCorpGroupConfigStorage()`）。
    fn wx_cp_corp_group_config_storage(&self) -> Arc<dyn WxCpCorpGroupConfigStorage>;

    /// 注入配置存储（对应 Java `setWxCpCorpGroupConfigStorage`）。
    fn set_wx_cp_corp_group_config_storage(&self, _config: Arc<dyn WxCpCorpGroupConfigStorage>) {}

    /// HTTP 客户端（对应 Java `getRequestHttp()`，reqwest 统一 HTTP，
    /// ADAPTED）。
    fn http_client(&self) -> &reqwest::Client;

    /// 注入企业微信服务（对应 Java `setWxCpService(WxCpService)`；用于
    /// 获取本企业 token 与小程序 session 转换）。
    fn set_wx_cp_service(&self, _service: Arc<dyn WxCpService>) {}

    /// 已注入的企业微信服务（对应 Java `wxCpService` 字段；默认 `None`）。
    fn wx_cp_service(&self) -> Option<Arc<dyn WxCpService>> {
        None
    }

    /// 互联企业的服务类对象（对应 Java `getLinkedCorpService()`）。
    fn linked_corp_service(&self) -> Arc<dyn WxCpLinkedCorpService>;

    // ---- 重试参数（对应 Java Base 字段） ----

    /// HTTP 请求重试间隔（毫秒），对应 Java Base 字段
    /// `retrySleepMillis`（默认 1000）。
    fn retry_sleep_millis(&self) -> i32 {
        1000
    }

    /// HTTP 请求最大重试次数，对应 Java Base 字段 `maxRetryTimes`
    /// （默认 5）。
    fn max_retry_times(&self) -> i32 {
        5
    }

    /// 设置当微信系统响应系统繁忙时，要等待多少
    /// `retrySleepMillis(ms) * 2^(重试次数 - 1)` 再发起重试（对应 Java
    /// `setRetrySleepMillis(int)`）。
    fn set_retry_sleep_millis(&self, _retry_sleep_millis: i32) {}

    /// 设置当微信系统响应系统繁忙时，最大重试次数（对应 Java
    /// `setMaxRetryTimes(int)`）。
    fn set_max_retry_times(&self, _max_retry_times: i32) {}

    // ---- corp access token（对应 Java 各方法） ----

    /// 更新企业 access token（对应 Java `updateCorpAccessToken(String,
    /// Integer, String, int)`：Java 原实现为空方法，此处转发配置存储
    /// 语义，ADAPTED）。
    fn update_corp_access_token(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        corp_access_token: &str,
        expires_in_seconds: i32,
    ) {
        self.wx_cp_corp_group_config_storage()
            .update_corp_access_token(corp_id, agent_id, corp_access_token, expires_in_seconds);
    }

    /// 获取企业 access token（对应 Java `getCorpAccessToken(String,
    /// Integer, Integer)`，不强制刷新）。
    async fn get_corp_access_token(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        business_type: Option<i32>,
    ) -> Result<String, WxErrorException> {
        self.get_corp_access_token_with_force(corp_id, agent_id, business_type, false)
            .await
    }

    /// 获取企业 access token（对应 Java `getCorpAccessToken(String,
    /// Integer, Integer, boolean)`，可强制刷新）。
    ///
    /// 双检锁（`corp_access_token_lock`）保证多线程同时刷新时只刷新
    /// 一次；token 经注入的 `WxCpService` POST
    /// `/cgi-bin/corpgroup/corp/gettoken` 获取（请求体
    /// `{corpid, agentid, business_type}`）。
    async fn get_corp_access_token_with_force(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        business_type: Option<i32>,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_corp_group_config_storage();
        if !config.is_corp_access_token_expired(corp_id, agent_id) && !force_refresh {
            return config
                .corp_access_token(corp_id, agent_id)
                .ok_or_else(|| WxErrorException::from_code(-99, "corp access token 为空"));
        }
        let lock = config.corp_access_token_lock(corp_id, agent_id);
        let _guard = lock.lock().await;
        // 拿到锁之后，再次判断一下最新的 token 是否过期，避免重刷
        if !config.is_corp_access_token_expired(corp_id, agent_id) && !force_refresh {
            return config
                .corp_access_token(corp_id, agent_id)
                .ok_or_else(|| WxErrorException::from_code(-99, "corp access token 为空"));
        }
        // 对应 Java synchronized(this)：经 WxCpService 获取本企业 token
        let wx_cp_service = self
            .wx_cp_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpService 未注入"))?;
        let body = serde_json::json!({
            "corpid": corp_id,
            "agentid": agent_id,
            "business_type": business_type,
        })
        .to_string();
        let url = wx_cp_service
            .wx_cp_config_storage()
            .api_url(url_corp_group::CORP_GET_TOKEN);
        let response_content = wx_cp_service.post(&url, &body).await?;
        let corp_token: WxAccessToken = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        config.update_corp_access_token(
            corp_id,
            agent_id,
            &corp_token.access_token,
            corp_token.expires_in,
        );
        config
            .corp_access_token(corp_id, agent_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "corp access token 为空"))
    }

    /// 获取企业 access token 实体（对应 Java
    /// `getCorpAccessTokenEntity(String, Integer, Integer)`，不强制刷新）。
    async fn get_corp_access_token_entity(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        business_type: Option<i32>,
    ) -> Result<WxAccessToken, WxErrorException> {
        self.get_corp_access_token_entity_with_force(corp_id, agent_id, business_type, false)
            .await
    }

    /// 获取企业 access token 实体（对应 Java
    /// `getCorpAccessTokenEntity(String, Integer, Integer, boolean)`）。
    ///
    /// 注意镜像 Java 原实现：刷新后直接返回配置存储中的实体（Java 未
    /// 将 `businessType` 传递到存储，实体以 corpId/agentId 为 key）。
    async fn get_corp_access_token_entity_with_force(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        business_type: Option<i32>,
        force_refresh: bool,
    ) -> Result<WxAccessToken, WxErrorException> {
        self.get_corp_access_token_with_force(corp_id, agent_id, business_type, force_refresh)
            .await?;
        Ok(self
            .wx_cp_corp_group_config_storage()
            .corp_access_token_entity(corp_id, agent_id))
    }

    /// 企业 access token 是否已过期（对应 Java
    /// `isCorpAccessTokenExpired(String, Integer)`）。
    fn is_corp_access_token_expired(&self, corp_id: &str, agent_id: Option<i32>) -> bool {
        self.wx_cp_corp_group_config_storage()
            .is_corp_access_token_expired(corp_id, agent_id)
    }

    /// 强制将企业 access token 过期掉（对应 Java
    /// `expireCorpAccessToken(String, Integer)`）。
    fn expire_corp_access_token(&self, corp_id: &str, agent_id: Option<i32>) {
        self.wx_cp_corp_group_config_storage()
            .expire_corp_access_token(corp_id, agent_id);
    }

    // ---- 通用 GET/POST 执行通道（对应 Java get/post 各重载） ----

    /// GET 请求（对应 Java `get(String, String, WxCpCorpGroupCorpGetTokenReq)`，
    /// 自动带 corp access token）。
    async fn get(
        &self,
        url: &str,
        query_param: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<String, WxErrorException> {
        self.get_without_corp_access_token(url, query_param, false, req)
            .await
    }

    /// GET 请求（对应 Java `get(String, String, boolean,
    /// WxCpCorpGroupCorpGetTokenReq)`：`true` 时忽略 corp access token）。
    async fn get_without_corp_access_token(
        &self,
        url: &str,
        query_param: &str,
        without_corp_access_token: bool,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<String, WxErrorException> {
        let executor = SimpleGetRequestExecutor::new(self.http_client().clone());
        crate::corpgroup::service::r#impl::base_wx_cp_cg_service_impl::execute_with_retry_cg(
            self,
            &executor,
            url,
            query_param.to_string(),
            without_corp_access_token,
            req,
        )
        .await
    }

    /// POST 请求（对应 Java `post(String, String, WxCpCorpGroupCorpGetTokenReq)`，
    /// 自动带 corp access token）。
    async fn post(
        &self,
        url: &str,
        post_data: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<String, WxErrorException> {
        self.post_without_corp_access_token(url, post_data, false, req)
            .await
    }

    /// POST 请求（对应 Java `post(String, String, boolean,
    /// WxCpCorpGroupCorpGetTokenReq)`：`true` 时忽略 corp access token）。
    async fn post_without_corp_access_token(
        &self,
        url: &str,
        post_data: &str,
        without_corp_access_token: bool,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<String, WxErrorException> {
        let executor = SimplePostRequestExecutor::new(self.http_client().clone());
        crate::corpgroup::service::r#impl::base_wx_cp_cg_service_impl::execute_with_retry_cg(
            self,
            &executor,
            url,
            post_data.to_string(),
            without_corp_access_token,
            req,
        )
        .await
    }

    // ---- 小程序 session（对应 Java getCorpTransferSession） ----

    /// 获取下级/下游企业小程序 session（对应 Java
    /// `getCorpTransferSession(String, String, WxCpCorpGroupCorpGetTokenReq)`，
    /// https://developer.work.weixin.qq.com/document/path/93355）。
    async fn get_corp_transfer_session(
        &self,
        user_id: &str,
        session_key: &str,
        req: &WxCpCorpGroupCorpGetTokenReq,
    ) -> Result<WxCpMaTransferSession, WxErrorException> {
        let wx_cp_service = self
            .wx_cp_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpService 未注入"))?;
        let body = serde_json::json!({
            "userid": user_id,
            "session_key": session_key,
        })
        .to_string();
        let url = wx_cp_service
            .wx_cp_config_storage()
            .api_url(url_corp_group::MA_TRANSFER_SESSION);
        let result = self.post(&url, &body, req).await?;
        WxCpMaTransferSession::from_json(&result).map_err(WxErrorException::Serde)
    }
}
