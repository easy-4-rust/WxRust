//! 微信小程序物流退货组件服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaExpressDeliveryReturnServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaExpressDeliveryReturnService;
use crate::bean::express::request::WxMaExpressDeliveryReturnAddRequest;
use crate::bean::express::result::WxMaExpressReturnInfoResult;
use crate::enums::g3_urls::url_g3_shop::express_delivery_return as return_url;

/// 微信小程序物流退货组件服务实现。
pub struct WxMaExpressDeliveryReturnServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaExpressDeliveryReturnServiceImpl {
    /// 构建物流退货组件服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaExpressDeliveryReturnService for WxMaExpressDeliveryReturnServiceImpl {
    /// 对应 Java `WxMaExpressDeliveryReturnServiceImpl.addDeliveryReturn`：
    /// POST `ADD_DELIVERY_RETURN_URL`（`request.toJson()`）后 `fromJson` 解析。
    async fn add_delivery_return(
        &self,
        request: &WxMaExpressDeliveryReturnAddRequest,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = request
            .to_json()
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&return_url::add_delivery_return_url(config.as_ref()), &body)
            .await?;
        WxMaExpressReturnInfoResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaExpressDeliveryReturnServiceImpl.getDeliveryReturn`：
    /// 构造 `{"return_id": returnId}` 后 POST `GET_DELIVERY_RETURN_URL` 并 `fromJson` 解析。
    async fn get_delivery_return(
        &self,
        return_id: &str,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "return_id": return_id }).to_string();
        let response = svc
            .post(&return_url::get_delivery_return_url(config.as_ref()), &body)
            .await?;
        WxMaExpressReturnInfoResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaExpressDeliveryReturnServiceImpl.unbindDeliveryReturn`：
    /// 构造 `{"return_id": returnId}` 后 POST `UNBIND_DELIVERY_RETURN_URL` 并 `fromJson` 解析。
    async fn unbind_delivery_return(
        &self,
        return_id: &str,
    ) -> Result<WxMaExpressReturnInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "return_id": return_id }).to_string();
        let response = svc
            .post(
                &return_url::unbind_delivery_return_url(config.as_ref()),
                &body,
            )
            .await?;
        WxMaExpressReturnInfoResult::from_json(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
