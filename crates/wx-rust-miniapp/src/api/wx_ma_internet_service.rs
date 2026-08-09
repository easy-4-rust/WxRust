//! 服务端网络相关服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaInternetService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::internet::WxMaInternetResponse;

/// 【小程序-服务端-网络】网络相关服务。
///
/// 文档：
/// <https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/internet/internet.getUserEncryptKey.html>
#[async_trait]
pub trait WxMaInternetService: Send + Sync {
    /// 获取用户 encryptKey（指定签名，对应 Java
    /// `WxMaInternetService.getUserEncryptKey(String, String, String)`，
    /// Java 中已标记 `@Deprecated`）。
    ///
    /// POST `/wxa/business/getuserencryptkey?openid=&signature=&sig_method=`，
    /// 请求体为空字符串。
    async fn get_user_encrypt_key_with_signature(
        &self,
        openid: &str,
        signature: &str,
        sig_method: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException>;

    /// 获取用户 encryptKey（对应 Java
    /// `WxMaInternetService.getUserEncryptKey(String, String)`）。
    ///
    /// signature 为以 **Base64 解码后的 sessionKey** 为密钥对空串做
    /// HmacSHA256 的十六进制大写结果；会获取用户最近 3 次的 key，每个 key 的
    /// 存活时间为 3600s。
    async fn get_user_encrypt_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<WxMaInternetResponse, WxErrorException>;
}
