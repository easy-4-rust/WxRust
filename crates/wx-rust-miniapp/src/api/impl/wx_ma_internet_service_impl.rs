//! 服务端网络相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaInternetServiceImpl`：
//! 全部方法委托门面默认实现（门面已镜像 Java Impl 的签名生成
//! `sha256("", sessionKey)` 与 URL 拼装/响应解析）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaInternetService;
use crate::bean::internet::WxMaInternetResponse;

/// 服务端网络相关服务实现。
pub struct WxMaInternetServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaInternetServiceImpl {
    /// 构建服务端网络服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaInternetService for WxMaInternetServiceImpl {
    /// 对应 Java `WxMaInternetServiceImpl.getUserEncryptKey(String, String,
    /// String)`（Java 中已标记 `@Deprecated`）。
    async fn get_user_encrypt_key_with_signature(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_user_encrypt_key_with_signature(openid, signature, sig_method)
            .await
    }

    /// 对应 Java `WxMaInternetServiceImpl.getUserEncryptKey(String, String)`。
    async fn get_user_encrypt_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        svc.get_user_encrypt_key(openid, session_key).await
    }
}
