//! 企业微信企业互联（corpgroup）配置存储。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.WxCpCorpGroupConfigStorage`：
//! 以「企业 ID + 应用 ID」为 key 的 corp access token 缓存（值 + 过期
//! 时刻 + 独立锁）+ 代理配置 + 本企业 corpId/agentId 配置。
//!
//! 说明：
//! - Java `getApacheHttpClientBuilder()` 为 Apache 客户端专属，Rust 以
//!   reqwest 单一后端承载（与 WxCpConfigStorage 同一 ADAPTED 说明）；
//! - 锁 key 镜像 Java `generateAccessTokenKey`：
//!   `corpId:agentId:reqCorpId:reqAgentId`。

use std::sync::Arc;

use tokio::sync::Mutex as AsyncMutex;

use wx_rust_common::bean::WxAccessToken;

/// 企业微信企业互联配置存储。
pub trait WxCpCorpGroupConfigStorage: Send + Sync {
    // ---- baseUrl / apiUrl（对应 Java setBaseApiUrl/getApiUrl） ----

    /// 设置企业微信服务器 baseUrl（对应 Java `setBaseApiUrl(String)`）。
    fn set_base_api_url(&self, base_url: &str);

    /// 读取企业微信 API Url（对应 Java `getApiUrl(String)`）。
    fn api_url(&self, path: &str) -> String {
        let base = self
            .base_api_url()
            .filter(|b| !b.is_empty())
            .unwrap_or_else(|| crate::enums::url_core::DEFAULT_CP_BASE_URL.to_string());
        format!("{base}{path}")
    }

    /// 读取自定义企业微信 API Url（对应 Java `getBaseApiUrl`）。
    fn base_api_url(&self) -> Option<String>;

    // ---- corp access token（对应 Java updateCorpAccessToken 等） ----

    /// 更新企业 access token（对应 Java
    /// `updateCorpAccessToken(String, Integer, String, int)`；
    /// 预留 200 秒提前过期）。
    fn update_corp_access_token(
        &self,
        corp_id: &str,
        agent_id: Option<i32>,
        corp_access_token: &str,
        expires_in_seconds: i32,
    );

    /// 企业 access token（对应 Java `getCorpAccessToken(String, Integer)`）。
    fn corp_access_token(&self, corp_id: &str, agent_id: Option<i32>) -> Option<String>;

    /// 企业 access token 实体（对应 Java `getCorpAccessTokenEntity`）：
    /// `expiresIn` 按 Java 公式 `(expireTime - now) / 1000 + 200`
    /// （未设置时 token 为空串、expire 按 0 计算）。
    fn corp_access_token_entity(&self, corp_id: &str, agent_id: Option<i32>) -> WxAccessToken;

    /// 企业 access token 是否已过期（对应 Java
    /// `isCorpAccessTokenExpired(String, Integer)`：不存在或已过期均为
    /// true）。
    fn is_corp_access_token_expired(&self, corp_id: &str, agent_id: Option<i32>) -> bool;

    /// 强制将企业 access token 过期掉（对应 Java
    /// `expireCorpAccessToken(String, Integer)`）。
    fn expire_corp_access_token(&self, corp_id: &str, agent_id: Option<i32>);

    // ---- 网络代理（对应 Java getHttpProxyHost 等） ----

    /// HTTP 代理主机（对应 Java `getHttpProxyHost()`）。
    fn http_proxy_host(&self) -> Option<String>;

    /// HTTP 代理端口（对应 Java `getHttpProxyPort()`）。
    fn http_proxy_port(&self) -> i32;

    /// HTTP 代理用户名（对应 Java `getHttpProxyUsername()`）。
    fn http_proxy_username(&self) -> Option<String>;

    /// HTTP 代理密码（对应 Java `getHttpProxyPassword()`）。
    fn http_proxy_password(&self) -> Option<String>;

    /// token 过期时是否自动刷新（对应 Java `autoRefreshToken()`，
    /// 恒 true）。
    fn auto_refresh_token(&self) -> bool {
        true
    }

    // ---- 本企业配置（对应 Java setCorpId/setAgentId/getCorpId/getAgentId） ----

    /// 设置微信企业号 corpId（对应 Java `setCorpId(String)`）。
    fn set_corp_id(&self, corp_id: &str);

    /// 微信企业号 corpId（对应 Java `getCorpId()`）。
    fn corp_id(&self) -> Option<String>;

    /// 设置微信企业号应用 ID（对应 Java `setAgentId(Integer)`）。
    fn set_agent_id(&self, agent_id: Option<i32>);

    /// 微信企业号应用 ID（对应 Java `getAgentId()`）。
    fn agent_id(&self) -> Option<i32>;

    /// 企业 access token 的锁（对应 Java
    /// `getCorpAccessTokenLock(String, Integer)`）。
    fn corp_access_token_lock(&self, corp_id: &str, agent_id: Option<i32>) -> Arc<AsyncMutex<()>>;
}
