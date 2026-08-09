//! 小程序交易组件-分享员服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopSharerServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopSharerService;
use crate::bean::shop::response::{
    WxMaShopSearchSharerResponse, WxMaShopSharerBindResponse, WxMaShopSharerDataSummaryResponse,
    WxMaShopSharerListResponse, WxMaShopSharerLiveOrderListResponse,
    WxMaShopSharerLiveSummaryListResponse, WxMaShopSharerUnbindResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_sharer as sharer_url;

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

/// 小程序交易组件-分享员服务实现。
pub struct WxMaShopSharerServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopSharerServiceImpl {
    /// 构建分享员服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopSharerService for WxMaShopSharerServiceImpl {
    /// 对应 Java `WxMaShopSharerServiceImpl.bindSharer`：
    /// 构造 `{"openids": [...]}` 后 POST `Sharer.BIND`。
    async fn bind_sharer(
        &self,
        openids: &[String],
    ) -> Result<WxMaShopSharerBindResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "openids",
            serde_json::to_value(openids).map_err(|e| WxErrorException::Serde(e.to_string()))?,
        )]);
        let response = svc
            .post(&sharer_url::bind_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.getSharerDataSummary`：
    /// 构造 `{"openid": ...}` 后 POST `GET_SHARER_DATA_SUMMARY`。
    async fn get_sharer_data_summary(
        &self,
        openid: &str,
    ) -> Result<WxMaShopSharerDataSummaryResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("openid", serde_json::Value::String(openid.to_string()))]);
        let response = svc
            .post(
                &sharer_url::get_sharer_data_summary_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.getSharerList`：
    /// 构造 `{"page", "page_size"}` 后 POST `GET_SHARER_LIST`。
    async fn get_sharer_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "page",
                page.map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&sharer_url::get_sharer_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.getSharerLiveOrderList`：
    /// 构造 `{"openid", "live_export_id", "page", "page_size"}` 后 POST
    /// `GET_SHARER_LIVE_ORDER_LIST`。
    async fn get_sharer_live_order_list(
        &self,
        openid: &str,
        live_export_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerLiveOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("openid", serde_json::Value::String(openid.to_string())),
            (
                "live_export_id",
                serde_json::Value::String(live_export_id.to_string()),
            ),
            (
                "page",
                page.map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(
                &sharer_url::get_sharer_live_order_list_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.getSharerLiveSummaryList`：
    /// 构造 `{"openid", "page", "page_size"}` 后 POST `GET_SHARER_LIVE_SUMMARY_LIST`。
    async fn get_sharer_live_summary_list(
        &self,
        openid: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerLiveSummaryListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("openid", serde_json::Value::String(openid.to_string())),
            (
                "page",
                page.map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "page_size",
                page_size
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(
                &sharer_url::get_sharer_live_summary_list_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.searchSharer`：
    /// 构造 `{"openid": ...}` 后 POST `SEARCH_SHARER`。
    async fn search_sharer(
        &self,
        openid: &str,
    ) -> Result<WxMaShopSearchSharerResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("openid", serde_json::Value::String(openid.to_string()))]);
        let response = svc
            .post(&sharer_url::search_sharer_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopSharerServiceImpl.unbindSharer`：
    /// 构造 `{"openids": [...]}` 后 POST `Sharer.UNBIND`。
    async fn unbind_sharer(
        &self,
        openids: &[String],
    ) -> Result<WxMaShopSharerUnbindResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "openids",
            serde_json::to_value(openids).map_err(|e| WxErrorException::Serde(e.to_string()))?,
        )]);
        let response = svc
            .post(&sharer_url::unbind_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
