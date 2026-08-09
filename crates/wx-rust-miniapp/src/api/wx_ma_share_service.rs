//! 分享信息相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShareService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::{WxMaGroupEnterInfo, WxMaShareInfo};

/// 分享信息相关操作服务。
#[async_trait]
pub trait WxMaShareService: Send + Sync {
    /// 解密分享敏感数据（对应 Java
    /// `WxMaShareService.getShareInfo(String, String, String)`）。
    ///
    /// 以 session_key 对 encryptedData 做 AES-128-CBC 解密后解析为
    /// `WxMaShareInfo`。
    async fn get_share_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaShareInfo, WxErrorException>;

    /// 解密群入口敏感数据（对应 Java
    /// `WxMaShareService.getGroupEnterInfo(String, String, String)`）。
    ///
    /// 对应 `wx.getGroupEnterInfo` 接口返回的 encryptedData 解密。
    async fn get_group_enter_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaGroupEnterInfo, WxErrorException>;
}
