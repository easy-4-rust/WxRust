//! 小程序交易组件-订单服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaShopOrderServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaShopOrderService;
use crate::bean::shop::WxMaShopOrderInfo;
use crate::bean::shop::request::WxMaShopOrderPayRequest;
use crate::bean::shop::response::{
    WxMaShopAddOrderResponse, WxMaShopBaseResponse, WxMaShopGetOrderListResponse,
    WxMaShopGetOrderResponse, WxMaShopGetPaymentParamsResponse,
};
use crate::enums::g3_urls::url_g3_shop::shop_order as order_url;

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

/// 毫秒时间戳格式化为 `yyyy-MM-dd HH:mm:ss`（对应 Java
/// `FastDateFormat.getInstance("yyyy-MM-dd HH:mm:ss")`；Rust 侧以 UTC 格式化，
/// 时间戳语义保持一致）。
fn format_date_time(millis: i64) -> String {
    chrono::DateTime::from_timestamp_millis(millis)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}

/// 小程序交易组件-订单服务实现。
pub struct WxMaShopOrderServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaShopOrderServiceImpl {
    /// 构建订单服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaShopOrderService for WxMaShopOrderServiceImpl {
    /// 对应 Java `WxMaShopOrderServiceImpl.checkScene`：
    /// POST `ORDER_CHECK_SCENE`（`{"scene": scene}`），返回响应 `is_matched` 布尔值。
    async fn check_scene(&self, scene: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("scene", serde_json::Value::from(scene))]);
        let response = svc
            .post(&order_url::check_scene_url(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("is_matched")
            .and_then(|v| v.as_bool())
            .ok_or_else(|| WxErrorException::from_code(-99, "响应缺少 is_matched 字段"))
    }

    /// 对应 Java `WxMaShopOrderServiceImpl.addOrder`：
    /// POST `ORDER_ADD`（序列化 `WxMaShopOrderInfo`）后解析响应。
    async fn add_order(
        &self,
        order_info: &WxMaShopOrderInfo,
    ) -> Result<WxMaShopAddOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = serde_json::to_string(order_info)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&order_url::order_add_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopOrderServiceImpl.orderPay`：
    /// POST `ORDER_PAY`（序列化 `WxMaShopOrderPayRequest`）后解析响应。
    async fn order_pay(
        &self,
        request: &WxMaShopOrderPayRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&order_url::order_pay_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopOrderServiceImpl.getOrder`：
    /// POST `ORDER_GET`（`{"order_id", "out_order_id", "openid"}`）后解析响应。
    async fn get_order(
        &self,
        order_id: Option<i64>,
        out_order_id: Option<&str>,
        openid: Option<&str>,
    ) -> Result<WxMaShopGetOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "order_id",
                order_id
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "out_order_id",
                out_order_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "openid",
                openid
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&order_url::order_get_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopOrderServiceImpl.getOrderList`：
    /// page 默认 1、page_size 默认 10、desc 为 true 时传 1 否则传 2，
    /// 起止时间按 `yyyy-MM-dd HH:mm:ss` 格式化（可空），POST `ORDER_GET_LIST`。
    async fn get_order_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        desc: bool,
        start_create_time: Option<i64>,
        end_create_time: Option<i64>,
    ) -> Result<WxMaShopGetOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("page", serde_json::Value::from(page.unwrap_or(1))),
            (
                "page_size",
                serde_json::Value::from(page_size.unwrap_or(10)),
            ),
            ("desc", serde_json::Value::from(if desc { 1 } else { 2 })),
            (
                "start_create_time",
                start_create_time
                    .map(|t| serde_json::Value::String(format_date_time(t)))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "end_create_time",
                end_create_time
                    .map(|t| serde_json::Value::String(format_date_time(t)))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&order_url::order_get_list_url(config.as_ref()), &body)
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 对应 Java `WxMaShopOrderServiceImpl.getPaymentParams`：
    /// POST `ORDER_GET_PAYMENT_PARAMS`（`{"order_id", "out_order_id", "openid"}`）。
    async fn get_payment_params(
        &self,
        order_id: Option<&str>,
        out_order_id: Option<&str>,
        openid: Option<&str>,
    ) -> Result<WxMaShopGetPaymentParamsResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "order_id",
                order_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "out_order_id",
                out_order_id
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "openid",
                openid
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(
                &order_url::order_get_payment_params_url(config.as_ref()),
                &body,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
