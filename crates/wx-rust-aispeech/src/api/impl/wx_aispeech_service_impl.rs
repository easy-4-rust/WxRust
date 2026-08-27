//! 智能对话服务实现。
//!
//! 对应 Java `me.chanjar.weixin.aispeech.api.impl.WxAispeechServiceImpl`：
//! 持有配置存储（含代理的 reqwest 客户端）与两个子服务。Java 的
//! `HttpComponentsClientBuilder` 初始化（`initHttp`）以 reqwest 客户端承载
//! （`PLATFORM_NA`：HttpClient 专属后端；代理在构建/重建客户端时一次性
//! 配置）。

use std::sync::{Arc, OnceLock, RwLock};

use crate::api::r#impl::{WxAispeechDialogServiceImpl, WxAispeechKnowledgeServiceImpl};
use crate::api::{WxAispeechDialogService, WxAispeechKnowledgeService, WxAispeechService};
use crate::config::WxAispeechConfigStorage;

/// 子服务集合（对应 Java `WxAispeechServiceImpl` 的 dialog/knowledge 字段）。
struct SubServices {
    dialog: Arc<dyn WxAispeechDialogService>,
    knowledge: Arc<dyn WxAispeechKnowledgeService>,
}

/// 智能对话服务实现（reqwest HTTP 后端）。
pub struct WxAispeechServiceImpl {
    config_storage: RwLock<Arc<dyn WxAispeechConfigStorage>>,
    client: RwLock<reqwest::Client>,
    sub_services: OnceLock<SubServices>,
}

impl WxAispeechServiceImpl {
    /// 构建服务（子服务注入 `Weak<dyn WxAispeechService>` 打破循环引用，
    /// 对应 Java `new WxAispeechDialogServiceImpl(this)`）。
    pub fn new_arc(config: Arc<dyn WxAispeechConfigStorage>) -> Arc<Self> {
        let client = build_http_client(config.as_ref());
        let arc = Arc::new(Self {
            config_storage: RwLock::new(config),
            client: RwLock::new(client),
            sub_services: OnceLock::new(),
        });
        // 先转 Arc<dyn WxAispeechService> 再降级为 Weak<dyn WxAispeechService>
        let dyn_arc: Arc<dyn WxAispeechService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc.sub_services.set(SubServices {
            dialog: Arc::new(WxAispeechDialogServiceImpl::new(weak.clone())),
            knowledge: Arc::new(WxAispeechKnowledgeServiceImpl::new(weak)),
        });
        arc
    }
}

/// 按配置的代理构建 reqwest 客户端（对应 Java `initHttp` 的
/// `httpProxyHost/Port` 判断；代理账号密码为 HttpComponents 专属
/// `PLATFORM_NA`，reqwest 不内建该支持）。
fn build_http_client(config: &dyn WxAispeechConfigStorage) -> reqwest::Client {
    let mut builder = reqwest::Client::builder();
    if let Some(host) = config.http_proxy_host()
        && !host.is_empty()
        && config.http_proxy_port() > 0
    {
        builder = builder.proxy(
            reqwest::Proxy::all(format!("http://{host}:{}", config.http_proxy_port()))
                .expect("代理地址合法"),
        );
    }
    builder.build().expect("HTTP 客户端构建失败")
}

impl WxAispeechService for WxAispeechServiceImpl {
    fn config_storage(&self) -> Arc<dyn WxAispeechConfigStorage> {
        self.config_storage.read().unwrap().clone()
    }

    fn set_config_storage(&self, config_storage: Arc<dyn WxAispeechConfigStorage>) {
        // 对应 Java `setConfigStorage` → `initHttp()`：先换配置再重建 HTTP 客户端
        *self.client.write().unwrap() = build_http_client(config_storage.as_ref());
        *self.config_storage.write().unwrap() = config_storage;
    }

    fn http_client(&self) -> reqwest::Client {
        self.client.read().unwrap().clone()
    }

    fn dialog_service(&self) -> Option<Arc<dyn WxAispeechDialogService>> {
        self.sub_services.get().map(|s| s.dialog.clone())
    }

    fn knowledge_service(&self) -> Option<Arc<dyn WxAispeechKnowledgeService>> {
        self.sub_services.get().map(|s| s.knowledge.clone())
    }
}
