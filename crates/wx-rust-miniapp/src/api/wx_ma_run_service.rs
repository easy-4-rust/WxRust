//! 微信运动相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaRunService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaRunStepInfo;

/// 微信运动相关操作服务。
#[async_trait]
pub trait WxMaRunService: Send + Sync {
    /// 解密运动数据（对应 Java
    /// `WxMaRunService.getRunStepInfo(String, String, String)`）。
    ///
    /// 对应 `wx.getWeRunData` 返回的 encryptedData 解密后解析
    /// `stepInfoList` 列表。
    async fn get_run_step_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<Vec<WxMaRunStepInfo>, WxErrorException>;
}
