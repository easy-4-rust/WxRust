//! 开放平台服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenServiceImpl`（继承
//! `WxOpenServiceHttpComponentsImpl` → `WxOpenServiceAbstractImpl`）：
//! 组合门面 trait 的默认实现 + 单一配置存储 + reqwest HTTP 后端。
//! 组件子服务以 `Weak<dyn WxOpenService>` 注入（对应 Java
//! `new WxOpenComponentServiceImpl(this)` 的循环引用，Rust 用弱引用打破）。

use std::sync::{Arc, OnceLock, RwLock};

use async_trait::async_trait;

use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::api::r#impl::{MinishopUploadRequestExecutor, WxOpenComponentServiceImpl};
use crate::api::{WxOpenComponentService, WxOpenService};
use crate::config::WxOpenConfigStorage;

/// 开放平台服务实现（reqwest HTTP 后端）。
pub struct WxOpenServiceImpl {
    client: reqwest::Client,
    config_storage: RwLock<Arc<dyn WxOpenConfigStorage>>,
    component_service: OnceLock<Arc<dyn WxOpenComponentService>>,
}

impl WxOpenServiceImpl {
    /// 构建服务（组件子服务注入 `Weak<dyn WxOpenService>` 打破循环引用）。
    ///
    /// # 参数
    /// - `config`：开放平台（第三方平台）配置存储
    pub fn new_arc(config: Arc<dyn WxOpenConfigStorage>) -> Arc<Self> {
        let arc = Arc::new(Self {
            client: reqwest::Client::new(),
            config_storage: RwLock::new(config),
            component_service: OnceLock::new(),
        });
        // 先转 Arc<dyn WxOpenService> 再降级为 Weak<dyn WxOpenService>
        let dyn_arc: Arc<dyn WxOpenService> = arc.clone();
        let weak = Arc::downgrade(&dyn_arc);
        let _ = arc
            .component_service
            .set(Arc::new(WxOpenComponentServiceImpl::new(weak)));
        arc
    }
}

#[async_trait]
impl WxOpenService for WxOpenServiceImpl {
    fn wx_open_component_service(&self) -> Option<Arc<dyn WxOpenComponentService>> {
        self.component_service.get().cloned()
    }

    fn wx_open_config_storage(&self) -> Arc<dyn WxOpenConfigStorage> {
        self.config_storage.read().unwrap().clone()
    }

    fn set_wx_open_config_storage(&self, wx_open_config_storage: Arc<dyn WxOpenConfigStorage>) {
        *self.config_storage.write().unwrap() = wx_open_config_storage;
        // Java `setWxOpenConfigStorage` 触发 `initHttp()` 重建 HTTP 客户端
        // （代理配置）；Rust reqwest Client 在构造时固定，代理/超时重建
        // 留待 Wave 1（ADAPTED）
    }

    fn http_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// 上传图片到小程序/开放平台素材库（对应 Java
    /// `uploadMinishopMediaFile(String url, File file)`）。
    ///
    /// Wave 2 实现：经 [`MinishopUploadRequestExecutor`]（reqwest multipart，
    /// 字段名 `media`，对应 Java `MinishopUploadRequestExecutor`）；
    /// ADAPTED：Java `File` 入参 → Rust 文件路径字符串。
    async fn upload_minishop_media_file(
        &self,
        url: &str,
        file_path: &str,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException> {
        MinishopUploadRequestExecutor::new(self.http_client().clone())
            .upload(url, file_path)
            .await
    }
}
