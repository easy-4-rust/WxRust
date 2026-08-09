//! 分享信息相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShareServiceImpl`：
//! 解密分享/群入口敏感数据后解析（Java 方法体内未使用 service，Rust 侧仅
//! 校验服务引用存活）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaShareService;
use crate::bean::{WxMaGroupEnterInfo, WxMaShareInfo};

/// 分享信息相关服务实现。
pub struct WxMaShareServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShareServiceImpl {
    /// 构建分享信息服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShareService for WxMaShareServiceImpl {
    /// 对应 Java `WxMaShareServiceImpl.getShareInfo`：
    /// `WxMaShareInfo.fromJson(WxMaCryptUtils.decrypt(...))`。
    async fn get_share_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaShareInfo, WxErrorException> {
        let _svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let decrypted =
            crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
                .map_err(WxErrorException::Io)?;
        WxMaShareInfo::from_json(&decrypted).map_err(WxErrorException::Serde)
    }

    /// 对应 Java `WxMaShareServiceImpl.getGroupEnterInfo`：
    /// `WxMaGroupEnterInfo.fromJson(WxMaCryptUtils.decrypt(...))`。
    async fn get_group_enter_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaGroupEnterInfo, WxErrorException> {
        let _svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let decrypted =
            crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
                .map_err(WxErrorException::Io)?;
        WxMaGroupEnterInfo::from_json(&decrypted).map_err(WxErrorException::Serde)
    }
}
