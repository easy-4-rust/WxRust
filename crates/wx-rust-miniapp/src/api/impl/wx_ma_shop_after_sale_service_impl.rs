//! 小程序交易组件-售后服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopAfterSaleServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopAfterSaleService;
use crate::bean::shop::request::{
    WxMaShopAcceptReturnRequest, WxMaShopAfterSaleAddRequest, WxMaShopAfterSaleGetRequest,
    WxMaShopAfterSaleListRequest, WxMaShopAfterSaleUpdateRequest,
    WxMaShopAfterSaleUploadReturnInfoRequest, WxMaShopEcAfterSaleGetRequest,
    WxMaShopEcAfterSaleUpdateRequest, WxMaShopUploadCerficatesRequest,
};
use crate::bean::shop::response::{
    WxMaShopAfterSaleAddResponse, WxMaShopAfterSaleGetResponse, WxMaShopAfterSaleListResponse,
    WxMaShopBaseResponse, WxMaShopEcAfterSaleGetResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_aftersale as aftersale_url;

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

/// 小程序交易组件-售后服务实现。
pub struct WxMaShopAfterSaleServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopAfterSaleServiceImpl {
    /// 构建售后服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopAfterSaleService for WxMaShopAfterSaleServiceImpl {
    /// 对应 Java `WxMaShopAfterSaleServiceImpl.add`：
    /// POST `AFTERSALE_ADD` 后校验 errcode 并解析响应。
    async fn add(
        &self,
        request: &WxMaShopAfterSaleAddRequest,
    ) -> Result<WxMaShopAfterSaleAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::add_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.get(WxMaShopAfterSaleGetRequest)`：
    /// POST `AFTERSALE_GET` 后校验 errcode 并解析响应。
    async fn get(
        &self,
        request: &WxMaShopAfterSaleGetRequest,
    ) -> Result<WxMaShopAfterSaleGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::get_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.get(WxMaShopEcAfterSaleGetRequest)`（EC 版）：
    /// POST `ECAFTERSALE_GET` 后校验 errcode 并解析响应。
    async fn get_ec(
        &self,
        request: &WxMaShopEcAfterSaleGetRequest,
    ) -> Result<WxMaShopEcAfterSaleGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::ec_get_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.update(WxMaShopAfterSaleUpdateRequest)`：
    /// POST `AFTERSALE_UPDATE` 后校验 errcode 并解析响应。
    async fn update(
        &self,
        request: &WxMaShopAfterSaleUpdateRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::update_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.update(WxMaShopEcAfterSaleUpdateRequest)`（EC 版）：
    /// POST `EC_AFTERSALE_UPDATE` 后校验 errcode 并解析响应。
    async fn update_ec(
        &self,
        request: &WxMaShopEcAfterSaleUpdateRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::ec_update_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.cancel`：
    /// 构造 `{"out_aftersale_id", "aftersale_id", "openid"}` 后 POST `AFTERSALE_CANCEL`。
    async fn cancel(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
        open_id: &str,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "out_aftersale_id",
                out_after_sale_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "aftersale_id",
                after_sale_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            ("openid", serde_json::Value::String(open_id.to_string())),
        ]);
        let response = svc
            .post(&aftersale_url::cancel_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.uploadReturnInfo`：
    /// POST `AFTERSALE_UPLOAD_RETURN_INFO` 后校验 errcode 并解析响应。
    async fn upload_return_info(
        &self,
        request: &WxMaShopAfterSaleUploadReturnInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &aftersale_url::upload_return_info_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.acceptRefund`：
    /// 构造 `{"out_aftersale_id", "aftersale_id"}` 后 POST `AFTERSALE_ACCEPT_REFUND`。
    async fn accept_refund(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "out_aftersale_id",
                out_after_sale_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "aftersale_id",
                after_sale_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&aftersale_url::accept_refund_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.acceptReturn`：
    /// POST `AFTERSALE_ACCEPT_RETURN` 后校验 errcode 并解析响应。
    async fn accept_return(
        &self,
        request: &WxMaShopAcceptReturnRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::accept_return_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.reject`：
    /// 构造 `{"out_aftersale_id", "aftersale_id"}` 后 POST `AFTERSALE_REJECT`。
    async fn reject(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "out_aftersale_id",
                out_after_sale_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "aftersale_id",
                after_sale_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&aftersale_url::reject_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.uploadCertificates`：
    /// POST `AFTERSALE_UPLOAD_CERTIFICATES` 后校验 errcode 并解析响应。
    async fn upload_certificates(
        &self,
        request: &WxMaShopUploadCerficatesRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(
                &aftersale_url::upload_certificates_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.updateDeadline`：
    /// 构造 `{"out_order_id", "order_id", "openid", "after_sale_deadline"}` 后
    /// POST `AFTERSALE_UPLOAD_DEADLINE`。
    async fn update_deadline(
        &self,
        out_order_id: Option<&str>,
        order_id: Option<i64>,
        openid: &str,
        after_sale_deadline: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "out_order_id",
                out_order_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "order_id",
                order_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            ("openid", serde_json::Value::String(openid.to_string())),
            (
                "after_sale_deadline",
                after_sale_deadline
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&aftersale_url::update_deadline_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopAfterSaleServiceImpl.list`：
    /// POST `AFTERSALE_GET_LIST` 后校验 errcode 并解析响应。
    async fn list(
        &self,
        request: &WxMaShopAfterSaleListRequest,
    ) -> Result<WxMaShopAfterSaleListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&aftersale_url::get_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
