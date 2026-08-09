//! 小程序物流助手服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaExpressServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaExpressService, WxMaService};
use crate::bean::{
    WxMaExpressAccount, WxMaExpressAddOrderRequest, WxMaExpressBindAccountRequest,
    WxMaExpressDelivery, WxMaExpressGetOrderRequest, WxMaExpressInfoResult,
    WxMaExpressOrderInfoResult, WxMaExpressPath, WxMaExpressPrinter,
    WxMaExpressPrinterUpdateRequest, WxMaExpressTestUpdateOrderRequest,
};
use crate::enums::url_g1_core::express as express_url;

/// 小程序物流助手服务实现。
pub struct WxMaExpressServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaExpressServiceImpl {
    /// 构建物流助手服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaExpressService for WxMaExpressServiceImpl {
    async fn get_all_delivery(&self) -> Result<Vec<WxMaExpressDelivery>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getAllDelivery`：GET `ALL_DELIVERY_URL` 后
        // `WxMaExpressDelivery.fromJson`（取 `data` 数组）
        let config = svc.wx_ma_config();
        let response = svc
            .get(&express_url::all_delivery_url(config.as_ref()), "")
            .await?;
        WxMaExpressDelivery::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_all_account(&self) -> Result<Vec<WxMaExpressAccount>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getAllAccount`：GET `ALL_ACCOUNT_URL` 后
        // `WxMaExpressAccount.fromJsonList`（取 `list` 数组）
        let config = svc.wx_ma_config();
        let response = svc
            .get(&express_url::all_account_url(config.as_ref()), "")
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("list")
            .ok_or_else(|| WxErrorException::from_code(-99, "缺少 list 字段"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn bind_account(
        &self,
        request: &WxMaExpressBindAccountRequest,
    ) -> Result<WxMaExpressInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `bindAccount`：POST `BIND_ACCOUNT_URL` 后
        // `WxMaExpressInfoResult.fromJson`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&express_url::bind_account_url(config.as_ref()), &body)
            .await?;
        WxMaExpressInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_quota(
        &self,
        request: &WxMaExpressBindAccountRequest,
    ) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getQuota`：POST `GET_QUOTA_URL` 后解析为
        // `WxMaExpressAccount` 并返回 `getQuotaNum()`（Java 返回装箱 Integer）
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&express_url::get_quota_url(config.as_ref()), &body)
            .await?;
        let account = WxMaExpressAccount::from_json(&response).map_err(WxErrorException::Serde)?;
        Ok(account.quota_num)
    }

    async fn update_printer(
        &self,
        request: &WxMaExpressPrinterUpdateRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `updatePrinter`：POST `UPDATE_PRINTER_URL`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&express_url::update_printer_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn get_printer(&self) -> Result<WxMaExpressPrinter, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getPrinter`：GET `GET_PRINTER_URL` 后
        // `WxMaExpressPrinter.fromJson`
        let config = svc.wx_ma_config();
        let response = svc
            .get(&express_url::get_printer_url(config.as_ref()), "")
            .await?;
        WxMaExpressPrinter::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn add_order(
        &self,
        request: &WxMaExpressAddOrderRequest,
    ) -> Result<WxMaExpressOrderInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `addOrder`：POST `ADD_ORDER_URL` 后
        // `WxMaExpressOrderInfoResult.fromJson`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&express_url::add_order_url(config.as_ref()), &body)
            .await?;
        WxMaExpressOrderInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn batch_get_order(
        &self,
        requests: &[WxMaExpressGetOrderRequest],
    ) -> Result<Vec<WxMaExpressOrderInfoResult>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `batchGetOrder`：请求体 `{"order_list": [...]}` 后
        // `WxMaExpressOrderInfoResult.toList`（取 `order_list` 数组）
        let config = svc.wx_ma_config();
        let body = serde_json::json!({ "order_list": requests }).to_string();
        let response = svc
            .post(&express_url::batch_get_order_url(config.as_ref()), &body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = json
            .get("order_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "缺少 order_list 字段"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn cancel_order(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `cancelOrder`：POST `CANCEL_ORDER_URL`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&express_url::cancel_order_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn get_order(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<WxMaExpressOrderInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getOrder`：POST `GET_ORDER_URL` 后
        // `WxMaExpressOrderInfoResult.fromJson`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&express_url::get_order_url(config.as_ref()), &body)
            .await?;
        WxMaExpressOrderInfoResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_path(
        &self,
        request: &WxMaExpressGetOrderRequest,
    ) -> Result<WxMaExpressPath, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getPath`：POST `GET_PATH_URL` 后 `WxMaExpressPath.fromJson`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&express_url::get_path_url(config.as_ref()), &body)
            .await?;
        WxMaExpressPath::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn test_update_order(
        &self,
        request: &WxMaExpressTestUpdateOrderRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `testUpdateOrder`：POST `TEST_UPDATE_ORDER_URL`
        let config = svc.wx_ma_config();
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        svc.post(&express_url::test_update_order_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }
}
