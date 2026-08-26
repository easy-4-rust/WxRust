//! 客服服务实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpKefuServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpKefuService, WxMpService};
use crate::bean::kefu::WxMpKefuMessage;
use crate::bean::kefu::request::{WxMpKfAccountRequest, WxMpKfSessionRequest};
use crate::bean::kefu::result::{
    WxMpKfList, WxMpKfMsgList, WxMpKfOnlineList, WxMpKfSessionGetResult, WxMpKfSessionList,
    WxMpKfSessionWaitCaseList,
};
use crate::enums::wx_mp_api_url::kefu as kefu_url;

/// 客服服务实现。
pub struct WxMpKefuServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpKefuServiceImpl {
    /// 构建客服服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 校验响应 errcode 是否为 0。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value
            .get("errcode")
            .map(|v| v.as_i64() == Some(0))
            .unwrap_or(false))
    }
}

#[async_trait]
impl WxMpKefuService for WxMpKefuServiceImpl {
    async fn send_kefu_message(
        &self,
        message: &WxMpKefuMessage,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = message.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&kefu_url::message_custom_send(config.as_ref()), &body)
            .await
    }

    async fn kf_list(&self) -> Result<WxMpKfList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc.get(&kefu_url::getkflist(config.as_ref()), "").await?;
        WxMpKfList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_online_list(&self) -> Result<WxMpKfOnlineList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&kefu_url::getonlinekflist(config.as_ref()), "")
            .await?;
        WxMpKfOnlineList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_account_add(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&kefu_url::kfaccount_add(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_account_update(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&kefu_url::kfaccount_update(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_account_invite_worker(
        &self,
        request: &WxMpKfAccountRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&kefu_url::kfaccount_invite_worker(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_account_del(&self, kf_account: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&kefu_url::kfaccount_del(config.as_ref(), kf_account), "")
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_session_create(
        &self,
        request: &WxMpKfSessionRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&kefu_url::kfsession_create(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_session_close(
        &self,
        request: &WxMpKfSessionRequest,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&kefu_url::kfsession_close(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn kf_session_get(
        &self,
        openid: &str,
    ) -> Result<WxMpKfSessionGetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&kefu_url::kfsession_get(config.as_ref(), openid), "")
            .await?;
        WxMpKfSessionGetResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_session_list(
        &self,
        kf_account: &str,
    ) -> Result<WxMpKfSessionList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&kefu_url::kfsession_list(config.as_ref(), kf_account), "")
            .await?;
        WxMpKfSessionList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_session_get_wait_case(
        &self,
    ) -> Result<WxMpKfSessionWaitCaseList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&kefu_url::kfsession_get_wait_case(config.as_ref()), "")
            .await?;
        WxMpKfSessionWaitCaseList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn kf_msg_list(
        &self,
        start_time: i64,
        end_time: i64,
        msg_id: i64,
        number: i32,
    ) -> Result<WxMpKfMsgList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        if number > 10000 {
            return Err(WxErrorException::from_code(
                -99,
                "非法参数请求，每次最多查询10000条记录！",
            ));
        }
        if start_time > end_time {
            return Err(WxErrorException::from_code(
                -99,
                "起始时间不能晚于结束时间！",
            ));
        }
        let body = serde_json::json!({
            "starttime": start_time,
            "endtime": end_time,
            "msgid": msg_id,
            "number": number
        });
        let response = svc
            .post(&kefu_url::getmsglist(config.as_ref()), &body.to_string())
            .await?;
        WxMpKfMsgList::from_json(&response).map_err(WxErrorException::Serde)
    }
}
