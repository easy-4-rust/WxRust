//! 用户信息相关操作接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaUserService`。

use std::collections::HashMap;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMaCode2VerifyInfoResult, WxMaJscode2SessionResult, WxMaPhoneNumberInfo, WxMaUserInfo,
};

/// 用户信息相关操作接口。
#[async_trait]
pub trait WxMaUserService: Send + Sync {
    /// 获取登录后的 session 信息（对应 Java `getSessionInfo(String)`）。
    async fn get_session_info(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException>;

    /// 解密用户敏感数据（对应 Java `getUserInfo(String, String, String)`）。
    async fn get_user_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaUserInfo, WxErrorException>;

    /// 上报用户数据后台接口（对应 Java `setUserStorage(Map, String, String)`）。
    async fn set_user_storage(
        &self,
        kv_map: &HashMap<String, String>,
        session_key: &str,
        openid: &str,
    ) -> Result<(), WxErrorException>;

    /// 解密用户手机号信息（对应 Java `getPhoneNoInfo(String, String, String)`，
    /// Java 已标记 `@Deprecated`）。
    async fn get_phone_no_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaPhoneNumberInfo, WxErrorException>;

    /// 通过 code 获取手机号（对应 Java `getPhoneNumber(String)`）。
    ///
    /// 响应无 `phone_info` 字段时返回 `None`（Java 返回 null）。
    async fn get_phone_number(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException>;

    /// 通过 code 获取手机号（对应 Java `getPhoneNoInfo(String)`，Java 已标记
    /// `@Deprecated`，委托 `getPhoneNumber`）。
    async fn get_phone_no_info_with_code(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException>;

    /// 验证用户信息完整性（对应 Java `checkUserInfo(String, String, String)`）。
    fn check_user_info(&self, session_key: &str, raw_data: &str, signature: &str) -> bool;

    /// 多端登录验证接口（对应 Java `getCode2VerifyInfo(String, String)`）。
    async fn get_code2_verify_info(
        &self,
        code: &str,
        checkcode: &str,
    ) -> Result<WxMaCode2VerifyInfoResult, WxErrorException>;

    /// 检查登录态（对应 Java `checkSessionKey(String, String)`）。
    ///
    /// 登录态有效时返回 `true`；已失效时微信服务端返回错误码（如 87009）
    /// 并以 `WxErrorException` 形式抛出。
    async fn check_session_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<bool, WxErrorException>;
}
