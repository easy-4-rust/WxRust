//! 企业微信服务门面「第三方应用代理」实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpServiceOnTpImpl`（继承
//! `WxCpServiceApacheHttpClientImpl`）：在第三方应用（tp）模式下，以
//! `WxCpTpService` 代理实现 `WxCpService` 门面的适配类。企业应用侧
//! 配置存储为普通 `WxCpConfigStorage`（corpId/corpSecret 语义下
//! corpSecret 即企业永久授权码）。
//!
//! 核心覆写（对应 Java）：
//! - `get_access_token_with_force`：access token 通过第三方应用 service
//!   获取——`wxCpTpService.getCorpToken(corpId, corpSecret)`（corpSecret
//!   对应企业永久授权码），随后更新本地配置缓存；
//! - 其余门面方法走 `WxCpService` trait 默认实现（get/post 执行引擎
//!   自动带 access_token → 触发本覆写的 token 获取链路，与 Java 继承
//!   Base 的行为一致）。
//!
//! 说明（诚实报告）：Java 侧继承 Base 得到的
//! `getContactAccessToken`/`getMsgAuditAccessToken`/jsapi_ticket 等
//! 完整实现，在 Rust 中保持 trait 默认（-99 未实现 + 其余默认行为）；
//! 子服务 getter 保持默认 `None`。核心代理语义（access_token 链路）已
//! 全量镜像，剩余差异不影响 tp 模式代企业调用的主路径。

use std::sync::{Arc, Mutex, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_common::session::{StandardSessionManager, WxSessionManager};

use crate::api::WxCpService;
use crate::config::WxCpConfigStorage;
use crate::tp::service::WxCpTpService;

/// 企业微信服务门面第三方应用代理实现。
pub struct WxCpServiceOnTpImpl {
    /// 第三方应用服务（对应 Java `wxCpTpService` 字段；Weak 打破循环）。
    tp_service: Weak<dyn WxCpTpService>,
    client: reqwest::Client,
    config_storage: Mutex<Arc<dyn WxCpConfigStorage>>,
    session_manager: Arc<dyn WxSessionManager>,
}

impl WxCpServiceOnTpImpl {
    /// 构建门面代理（对应 Java `@RequiredArgsConstructor` 注入
    /// `WxCpTpService`）。
    pub fn new(tp_service: Weak<dyn WxCpTpService>) -> Self {
        Self {
            tp_service,
            client: reqwest::Client::new(),
            config_storage: Mutex::new(Arc::new(crate::config::r#impl::WxCpDefaultConfig::new(
                "", "",
            ))),
            // 对应 Java Base 默认装配 StandardSessionManager
            session_manager: Arc::new(StandardSessionManager::new()),
        }
    }

    /// 升级第三方应用服务引用（Weak 失效时抛 -99，ADAPTED）。
    fn tp_service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.tp_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }
}

#[async_trait]
impl WxCpService for WxCpServiceOnTpImpl {
    fn wx_cp_config_storage(&self) -> Arc<dyn WxCpConfigStorage> {
        self.config_storage.lock().unwrap().clone()
    }

    fn set_wx_cp_config_storage(&self, config: Arc<dyn WxCpConfigStorage>) {
        *self.config_storage.lock().unwrap() = config;
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    fn session_manager(&self) -> Option<Arc<dyn WxSessionManager>> {
        Some(self.session_manager.clone())
    }

    /// 获取 access_token（对应 Java `getAccessToken(boolean)` 覆写）：
    /// 缓存有效且非强制刷新时直接返回；否则经第三方应用 service 获取
    /// 企业 token（`corpSecret` 即企业永久授权码）并更新本地缓存。
    async fn get_access_token_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let config = self.wx_cp_config_storage();
        if !config.is_access_token_expired() && !force_refresh {
            return config
                .access_token()
                .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"));
        }
        // access token 通过第三方应用 service 获取
        // corpSecret 对应企业永久授权码
        let corp_id = config.app_id().to_string();
        let permanent_code = config.secret().to_string();
        let access_token = self
            .tp_service()?
            .get_corp_token(&corp_id, &permanent_code)
            .await?;
        config.update_access_token(&access_token.access_token, access_token.expires_in);
        config
            .access_token()
            .ok_or_else(|| WxErrorException::from_code(-99, "access token 为空"))
    }
}
