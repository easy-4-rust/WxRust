//! 用户服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaUserServiceImpl`。
//! 各方法委托门面 `WxMaService`（Rust 门面已承载 BaseWxMaServiceImpl 的
//! 登录会话/AES 解密/HmacSHA256 签名等实现，与 Java 委托 `service` 同一语义）。

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaService, WxMaUserService};
use crate::bean::{
    WxMaCode2VerifyInfoResult, WxMaJscode2SessionResult, WxMaPhoneNumberInfo, WxMaUserInfo,
};

/// 用户服务实现。
pub struct WxMaUserServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaUserServiceImpl {
    /// 构建用户服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaUserService for WxMaUserServiceImpl {
    async fn get_session_info(
        &self,
        js_code: &str,
    ) -> Result<WxMaJscode2SessionResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getSessionInfo` 委托 `service.jsCode2SessionInfo(jsCode)`
        svc.get_session_info(js_code).await
    }

    async fn get_user_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaUserInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getUserInfo`：`WxMaUserInfo.fromJson(WxMaCryptUtils.decrypt(...))`
        svc.get_user_info(session_key, encrypted_data, iv_str).await
    }

    async fn set_user_storage(
        &self,
        kv_map: &HashMap<String, String>,
        session_key: &str,
        openid: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `setUserStorage`：`kv_list` 数组 + HmacSHA256 签名
        // （`SignUtils.createHmacSha256Sign(params, sessionKey)`）
        svc.set_user_storage(kv_map, session_key, openid).await
    }

    async fn get_phone_no_info(
        &self,
        session_key: &str,
        encrypted_data: &str,
        iv_str: &str,
    ) -> Result<WxMaPhoneNumberInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getPhoneNoInfo(sessionKey, encryptedData, ivStr)`：
        // `WxMaPhoneNumberInfo.fromJson(WxMaCryptUtils.decrypt(...))`
        svc.get_phone_no_info(session_key, encrypted_data, iv_str)
            .await
    }

    async fn get_phone_number(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getPhoneNumber`：POST `/wxa/business/getuserphonenumber`，
        // 响应无 `phone_info` 时返回 null（Rust 以 None 表达）
        svc.get_phone_number(code).await
    }

    async fn get_phone_no_info_with_code(
        &self,
        code: &str,
    ) -> Result<Option<WxMaPhoneNumberInfo>, WxErrorException> {
        // Java `getPhoneNoInfo(String code)`（@Deprecated）委托 `getPhoneNumber`
        self.get_phone_number(code).await
    }

    fn check_user_info(&self, session_key: &str, raw_data: &str, signature: &str) -> bool {
        let svc = match self.service.upgrade() {
            Some(svc) => svc,
            None => return false,
        };
        // Java `checkUserInfo`：`sha1Hex(rawData + sessionKey)` 与 signature 比较
        svc.check_user_info(session_key, raw_data, signature)
    }

    async fn get_code2_verify_info(
        &self,
        code: &str,
        checkcode: &str,
    ) -> Result<WxMaCode2VerifyInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getCode2VerifyInfo`：POST `{"code":..., "checkcode":...}`
        svc.get_code2_verify_info(code, checkcode).await
    }

    async fn check_session_key(
        &self,
        openid: &str,
        session_key: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `checkSessionKey`：HmacSHA256 签名后 GET，请求成功恒返回 true
        svc.check_session_key(openid, session_key).await
    }
}
