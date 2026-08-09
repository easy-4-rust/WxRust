//! 微信小店合作账号服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxStoreCooperationServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_store_cooperation_service::WxStoreCooperationService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::cooperation::{
    CooperationListResponse, CooperationQrCodeResponse, CooperationSharerParam,
    CooperationStatusResponse,
};
use crate::enums::url_cooperation::{
    CANCEL_COOPERATION_URL, GENERATE_QRCODE_COOPERATION_URL, GET_COOPERATION_STATUS_URL,
    LIST_COOPERATION_URL, UNBIND_COOPERATION_URL,
};

/// 微信小店合作账号服务实现（对应 Java `WxStoreCooperationServiceImpl`）。
pub struct WxStoreCooperationServiceImpl {
    /// 微信小店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxStoreCooperationServiceImpl {
    /// 构建合作账号服务（对应 Java `new WxStoreCooperationServiceImpl(storeService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `storeService.post` +
    /// `ResponseUtils.decode`；errcode 校验由执行引擎完成，同 Java 语义）。
    async fn post_as<T>(
        svc: &dyn WxChannelService,
        url: &str,
        post_data: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_data).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxStoreCooperationService for WxStoreCooperationServiceImpl {
    /// 获取合作账号列表（对应 Java `listCooperation(Integer)`，请求体
    /// `{"sharer_type":N}` 逐字对齐）。
    async fn list_cooperation(
        &self,
        sharer_type: Option<i32>,
    ) -> Result<CooperationListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param_json = format!("{{\"sharer_type\":{}}}", sharer_type.unwrap_or(0));
        Self::post_as(svc.as_ref(), LIST_COOPERATION_URL, &param_json).await
    }

    /// 获取合作账号状态（对应 Java `getCooperationStatus`，内部构造
    /// `CooperationSharerParam`，请求体 `{"sharer_id":"..","sharer_type":N}`）。
    async fn get_cooperation_status(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<CooperationStatusResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = CooperationSharerParam {
            sharer_id,
            sharer_type: sharer_type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_COOPERATION_STATUS_URL, &req_json).await
    }

    /// 生成合作账号邀请二维码（对应 Java `generateQrCode`）。
    async fn generate_qr_code(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<CooperationQrCodeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = CooperationSharerParam {
            sharer_id,
            sharer_type: sharer_type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GENERATE_QRCODE_COOPERATION_URL, &req_json).await
    }

    /// 取消合作账号邀请（对应 Java `cancelInvitation`）。
    async fn cancel_invitation(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = CooperationSharerParam {
            sharer_id,
            sharer_type: sharer_type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), CANCEL_COOPERATION_URL, &req_json).await
    }

    /// 解绑合作账号（对应 Java `unbind`）。
    async fn unbind(
        &self,
        sharer_id: String,
        sharer_type: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = CooperationSharerParam {
            sharer_id,
            sharer_type: sharer_type.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), UNBIND_COOPERATION_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取合作账号列表：字面量请求体 `{"sharer_type":N}` 与响应解析
    /// （对应 Java `listCooperation`）。
    #[tokio::test]
    async fn test_list_cooperation() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","data_list":[{"sharer_id":"gh_x","sharer_type":2}]}"#,
        );
        let sub = WxStoreCooperationServiceImpl::new(weak);
        let resp = sub.list_cooperation(Some(2)).await.unwrap();
        assert_eq!(resp.data_list.len(), 1);
        assert_eq!(resp.data_list[0].sharer_id, "gh_x");
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, LIST_COOPERATION_URL);
        assert_eq!(body, r#"{"sharer_type":2}"#);
    }

    /// 解绑合作账号：`CooperationSharerParam` 序列化请求体与响应解析
    /// （对应 Java `unbind`）。
    #[tokio::test]
    async fn test_unbind() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxStoreCooperationServiceImpl::new(weak);
        let resp = sub.unbind("gh_abc123".to_string(), Some(2)).await.unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, UNBIND_COOPERATION_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["sharer_id"], "gh_abc123");
        assert_eq!(json["sharer_type"], 2);
    }
}
