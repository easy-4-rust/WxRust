//! 小程序客服管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaKefuServiceImpl`。
//! 接口地址为 Java Impl 内联常量（未收敛进 `WxMaApiUrlConstants`），
//! 对应 URL 函数见 `enums::url_g1_core::kefu`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaKefuService, WxMaService};
use crate::bean::kefu::request::{WxMaKfAccountRequest, WxMaKfSessionRequest};
use crate::bean::kefu::{WxMaKfList, WxMaKfSession, WxMaKfSessionList};
use crate::enums::url_g1_core::kefu as kefu_url;

/// 小程序客服管理服务实现。
pub struct WxMaKefuServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaKefuServiceImpl {
    /// 构建客服管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaKefuService for WxMaKefuServiceImpl {
    async fn kf_list(&self) -> Result<WxMaKfList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfList`：GET `KFLIST_GET_URL` 后 `WxMaKfList.fromJson`
        let config = svc.wx_ma_config();
        let response = svc
            .get(&kefu_url::get_kf_list_url(config.as_ref()), "")
            .await?;
        WxMaKfList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_account_add(
        &self,
        request: &WxMaKfAccountRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfAccountAdd`：POST `KFACCOUNT_ADD_URL`，响应非 null 即返回 true
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&kefu_url::kf_account_add_url(config.as_ref()), &body)
            .await?;
        Ok(true)
    }

    async fn kf_account_update(
        &self,
        request: &WxMaKfAccountRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfAccountUpdate`：POST `KFACCOUNT_UPDATE_URL`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&kefu_url::kf_account_update_url(config.as_ref()), &body)
            .await?;
        Ok(true)
    }

    async fn kf_account_del(&self, kf_account: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfAccountDel`：GET `KFACCOUNT_DEL_URL`（`kf_account=%s` 拼入 URL）
        let config = svc.wx_ma_config();
        let url = kefu_url::kf_account_del_url(config.as_ref(), kf_account);
        svc.get(&url, "").await?;
        Ok(true)
    }

    async fn kf_session_create(
        &self,
        openid: &str,
        kf_account: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfSessionCreate`：构造 `WxMaKfSessionRequest` 后 POST
        // `KFSESSION_CREATE_URL`
        let config = svc.wx_ma_config();
        let request = WxMaKfSessionRequest {
            kf_account: kf_account.to_string(),
            openid: openid.to_string(),
        };
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&kefu_url::kf_session_create_url(config.as_ref()), &body)
            .await?;
        Ok(true)
    }

    async fn kf_session_close(
        &self,
        openid: &str,
        kf_account: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfSessionClose`：构造 `WxMaKfSessionRequest` 后 POST
        // `KFSESSION_CLOSE_URL`
        let config = svc.wx_ma_config();
        let request = WxMaKfSessionRequest {
            kf_account: kf_account.to_string(),
            openid: openid.to_string(),
        };
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&kefu_url::kf_session_close_url(config.as_ref()), &body)
            .await?;
        Ok(true)
    }

    async fn kf_session_get(&self, openid: &str) -> Result<WxMaKfSession, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfSessionGet`：GET `KFSESSION_GET_URL`（`openid=%s` 拼入 URL）
        let config = svc.wx_ma_config();
        let url = kefu_url::kf_session_get_url(config.as_ref(), openid);
        let response = svc.get(&url, "").await?;
        WxMaKfSession::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_session_list(
        &self,
        kf_account: &str,
    ) -> Result<WxMaKfSessionList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `kfSessionList`：GET `KFSESSION_LIST_URL`（`kf_account=%s` 拼入 URL）
        let config = svc.wx_ma_config();
        let url = kefu_url::kf_session_list_url(config.as_ref(), kf_account);
        let response = svc.get(&url, "").await?;
        WxMaKfSessionList::from_json(&response).map_err(WxErrorException::Serde)
    }
}
