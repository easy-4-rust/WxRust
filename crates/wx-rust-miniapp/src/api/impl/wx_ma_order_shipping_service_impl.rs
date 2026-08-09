//! 发货信息管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaOrderShippingServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaOrderShippingService;
use crate::bean::shop::request::shipping::{
    WxMaOrderCombinedShippingInfoUploadRequest, WxMaOrderShippingInfoGetListRequest,
    WxMaOrderShippingInfoGetRequest, WxMaOrderShippingInfoNotifyConfirmRequest,
    WxMaOrderShippingInfoUploadRequest,
};
use crate::bean::shop::response::{
    WxMaOrderShippingITMCCompletedResult, WxMaOrderShippingInfoBaseResponse,
    WxMaOrderShippingInfoGetListResponse, WxMaOrderShippingInfoGetResponse,
    WxMaOrderShippingIsTradeManagedResponse,
};
use crate::enums::g3_urls::url_g3_shop::order_shipping as order_shipping_url;

/// 构建 JSON 对象（跳过空值，对应 Java `GsonHelper.buildJsonObject`）。
fn build_json(pairs: &[(&str, serde_json::Value)]) -> String {
    let mut map = serde_json::Map::new();
    for (key, value) in pairs {
        if !value.is_null() {
            map.insert((*key).to_string(), value.clone());
        }
    }
    serde_json::to_string(&serde_json::Value::Object(map)).unwrap_or_else(|_| "{}".to_string())
}

/// 发货信息管理服务实现。
pub struct WxMaOrderShippingServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaOrderShippingServiceImpl {
    /// 构建发货信息管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 通用请求（对应 Java `WxMaOrderShippingServiceImpl.request`）：
    /// POST 后校验 errcode 并解析为指定类型。
    async fn request<T>(svc: &dyn WxMaService, url: &str, body: &str) -> Result<T, WxErrorException>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = svc.post(url, body).await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaOrderShippingService for WxMaOrderShippingServiceImpl {
    /// 对应 Java `WxMaOrderShippingServiceImpl.isTradeManaged`：
    /// 构造 `{"appid": appId}` 后 POST `IS_TRADE_MANAGED`。
    async fn is_trade_managed(
        &self,
        app_id: &str,
    ) -> Result<WxMaOrderShippingIsTradeManagedResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("appid", serde_json::Value::String(app_id.to_string()))]);
        Self::request(
            svc.as_ref(),
            &order_shipping_url::is_trade_managed_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.upload(WxMaOrderShippingInfoUploadRequest)`：
    /// POST `UPLOAD_SHIPPING_INFO`（序列化请求）。
    async fn upload(
        &self,
        request: &WxMaOrderShippingInfoUploadRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::request(
            svc.as_ref(),
            &order_shipping_url::upload_shipping_info_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.upload(WxMaOrderCombinedShippingInfoUploadRequest)`
    /// （合单录入）：POST `UPLOAD_COMBINED_SHIPPING_INFO`。
    async fn upload_combined(
        &self,
        request: &WxMaOrderCombinedShippingInfoUploadRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::request(
            svc.as_ref(),
            &order_shipping_url::upload_combined_shipping_info_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.get`：
    /// POST `GET_SHIPPING_INFO`（序列化请求）查询订单发货状态。
    async fn get(
        &self,
        request: &WxMaOrderShippingInfoGetRequest,
    ) -> Result<WxMaOrderShippingInfoGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::request(
            svc.as_ref(),
            &order_shipping_url::get_shipping_info_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.getList`：
    /// POST `GET_SHIPPING_INFO_LIST`（序列化请求）查询订单列表。
    async fn get_list(
        &self,
        request: &WxMaOrderShippingInfoGetListRequest,
    ) -> Result<WxMaOrderShippingInfoGetListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::request(
            svc.as_ref(),
            &order_shipping_url::get_shipping_info_list_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.notifyConfirmReceive`：
    /// POST `NOTIFY_CONFIRM_RECEIVE`（序列化请求）提醒用户确认收货。
    async fn notify_confirm_receive(
        &self,
        request: &WxMaOrderShippingInfoNotifyConfirmRequest,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::request(
            svc.as_ref(),
            &order_shipping_url::notify_confirm_receive_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.setMsgJumpPath`：
    /// 构造 `{"path": path}` 后 POST `SET_MSG_JUMP_PATH`。
    async fn set_msg_jump_path(
        &self,
        path: &str,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("path", serde_json::Value::String(path.to_string()))]);
        Self::request(
            svc.as_ref(),
            &order_shipping_url::set_msg_jump_path_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.isTradeManagementConfirmationCompleted`：
    /// 构造 `{"appid": appId}` 后 POST `IS_TRADE_MANAGEMENT_CONFIRMATION_COMPLETED`。
    async fn is_trade_management_confirmation_completed(
        &self,
        app_id: &str,
    ) -> Result<WxMaOrderShippingITMCCompletedResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("appid", serde_json::Value::String(app_id.to_string()))]);
        Self::request(
            svc.as_ref(),
            &order_shipping_url::is_trade_management_confirmation_completed_url(config.as_ref()),
            &body,
        )
        .await
    }

    /// 对应 Java `WxMaOrderShippingServiceImpl.opSpecialOrder`：
    /// 构造 `{"order_id", "type", "delay_to"}` 后 POST `OP_SPECIAL_ORDER`。
    async fn op_special_order(
        &self,
        order_id: &str,
        r#type: i32,
        delay_to: Option<i64>,
    ) -> Result<WxMaOrderShippingInfoBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("order_id", serde_json::Value::String(order_id.to_string())),
            ("type", serde_json::Value::from(r#type)),
            (
                "delay_to",
                delay_to
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        Self::request(
            svc.as_ref(),
            &order_shipping_url::op_special_order_url(config.as_ref()),
            &body,
        )
        .await
    }
}
