//! 微信运动相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaRunServiceImpl`：
//! 解密运动数据后解析 `stepInfoList` 列表（Java 方法体内未使用 service，
//! Rust 侧仅校验服务引用存活）。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaRunService;
use crate::bean::WxMaRunStepInfo;

/// 微信运动相关服务实现。
pub struct WxMaRunServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaRunServiceImpl {
    /// 构建微信运动服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaRunService for WxMaRunServiceImpl {
    /// 对应 Java `WxMaRunServiceImpl.getRunStepInfo`：
    /// `WxMaRunStepInfo.fromJson(WxMaCryptUtils.decrypt(...))`。
    async fn get_run_step_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<Vec<WxMaRunStepInfo>, WxErrorException> {
        let _svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let decrypted =
            crate::util::crypto::wx_ma_crypt_utils::decrypt(session_key, encrypted_data, iv_str)
                .map_err(WxErrorException::Io)?;
        WxMaRunStepInfo::from_json(&decrypted).map_err(WxErrorException::Serde)
    }
}
