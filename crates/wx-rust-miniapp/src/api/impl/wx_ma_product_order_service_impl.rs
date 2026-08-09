//! 小程序交易组件-标准版-商品订单服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaProductOrderServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaProductOrderService;
use crate::bean::product::{
    WxMiniBatchGetAfterSaleOrderResponse, WxMiniGetAfterSaleOrderResponse,
    WxMiniOrderDeliveryRequest, WxMinishopOrderDetailResponse, WxMinishopOrderListResponse,
};
use crate::bean::shop::response::WxMaShopBaseResponse;
use crate::enums::g3_urls::url_g3_shop::product as product_url;

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

/// 小程序交易组件-标准版-商品订单服务实现。
pub struct WxMaProductOrderServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaProductOrderServiceImpl {
    /// 构建标准版商品订单服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaProductOrderService for WxMaProductOrderServiceImpl {
    /// 对应 Java `WxMaProductOrderServiceImpl.getOrderList`：
    /// POST `PRODUCT_ORDER_GET_LIST`（8 个可选字段）后解析响应并校验 errCode。
    async fn get_order_list(
        &self,
        start_create_time: Option<&str>,
        end_create_time: Option<&str>,
        start_update_time: Option<&str>,
        end_update_time: Option<&str>,
        status: Option<i32>,
        page: Option<i32>,
        page_size: Option<i32>,
        source: Option<i32>,
    ) -> Result<WxMinishopOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            (
                "start_create_time",
                start_create_time
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "end_create_time",
                end_create_time
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "start_update_time",
                start_update_time
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "end_update_time",
                end_update_time
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .unwrap_or(serde_json::Value::Null),
            ),
            (
                "status",
                status
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
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
            (
                "source",
                source
                    .map(serde_json::Value::from)
                    .unwrap_or(serde_json::Value::Null),
            ),
        ]);
        let response = svc
            .post(&product_url::order::get_list_url(config.as_ref()), &body)
            .await?;
        let parsed: WxMinishopOrderListResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.getOrderDetail`：
    /// 构造 `{"order_id": orderId}` 后 POST `PRODUCT_ORDER_DETAIL_URL`。
    async fn get_order_detail(
        &self,
        order_id: i64,
    ) -> Result<WxMinishopOrderDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[("order_id", serde_json::Value::from(order_id))]);
        let response = svc
            .post(&product_url::order::detail_url(config.as_ref()), &body)
            .await?;
        let parsed: WxMinishopOrderDetailResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.changeMerchantNotes`：
    /// 构造 `{"order_id", "merchant_notes"}` 后 POST
    /// `PRODUCT_ORDER_CHANGE_MERCHANT_NOTES_URL`；Java 无返回值。
    async fn change_merchant_notes(
        &self,
        order_id: i64,
        merchant_notes: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("order_id", serde_json::Value::from(order_id)),
            (
                "merchant_notes",
                serde_json::Value::String(merchant_notes.to_string()),
            ),
        ]);
        let response = svc
            .post(
                &product_url::order::change_merchant_notes_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMaShopBaseResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(())
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.deliverySend`：
    /// POST `PRODUCT_DELIVERY_SEND`（序列化 `WxMiniOrderDeliveryRequest`）。
    async fn delivery_send(
        &self,
        request: &WxMiniOrderDeliveryRequest,
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
                &product_url::order::delivery_send_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMaShopBaseResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.getAfterSaleOrder`：
    /// 构造 `{"after_sale_order_id": ...}` 后 POST `GET_AFTER_SALE_ORDER`。
    async fn get_after_sale_order(
        &self,
        after_sale_order_id: i64,
    ) -> Result<WxMiniGetAfterSaleOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "after_sale_order_id",
            serde_json::Value::from(after_sale_order_id),
        )]);
        let response = svc
            .post(
                &product_url::order::get_after_sale_order_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMiniGetAfterSaleOrderResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.batchGetAfterSaleOrder`：
    /// 构造 `{"after_sale_order_id_list": [...]}` 后 POST
    /// `BATCH_GET_AFTER_SALE_ORDER`；Java 在售后单列表为 null 时抛
    /// `WxError(errCode, "售后查询不存在")`（Rust 以空列表表达）。
    async fn batch_get_after_sale_order(
        &self,
        after_sale_order_id_list: &[i64],
    ) -> Result<WxMiniBatchGetAfterSaleOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[(
            "after_sale_order_id_list",
            serde_json::to_value(after_sale_order_id_list)
                .map_err(|e| WxErrorException::Serde(e.to_string()))?,
        )]);
        let response = svc
            .post(
                &product_url::order::batch_get_after_sale_order_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMiniBatchGetAfterSaleOrderResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.after_sale_order_list.is_empty() {
            return Err(WxErrorException::from_code(
                parsed.err_code,
                "售后查询不存在",
            ));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.afterSaleAccept`：
    /// 构造 `{"order_id", "address_id"}` 后 POST `AFTER_SALE_ACCEPT_APPLY`。
    async fn after_sale_accept(
        &self,
        order_id: i64,
        address_id: i64,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("order_id", serde_json::Value::from(order_id)),
            ("address_id", serde_json::Value::from(address_id)),
        ]);
        let response = svc
            .post(
                &product_url::order::after_sale_accept_apply_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMaShopBaseResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }

    /// 对应 Java `WxMaProductOrderServiceImpl.afterSaleReject`：
    /// 构造 `{"order_id", "reject_reason"}` 后 POST `AFTER_SALE_REJECT_APPLY`
    /// （Java 入参 afterSaleOrderId 以 `order_id` 键提交）。
    async fn after_sale_reject(
        &self,
        after_sale_order_id: i64,
        reject_reason: &str,
    ) -> Result<WxMaShopBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = build_json(&[
            ("order_id", serde_json::Value::from(after_sale_order_id)),
            (
                "reject_reason",
                serde_json::Value::String(reject_reason.to_string()),
            ),
        ]);
        let response = svc
            .post(
                &product_url::order::after_sale_reject_apply_url(config.as_ref()),
                &body,
            )
            .await?;
        let parsed: WxMaShopBaseResponse =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if parsed.err_code != 0 {
            return Err(WxErrorException::from_code(parsed.err_code, parsed.err_msg));
        }
        Ok(parsed)
    }
}
