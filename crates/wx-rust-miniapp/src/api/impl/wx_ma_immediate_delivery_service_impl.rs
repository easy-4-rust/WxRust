//! 微信小程序即时配送服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaImmediateDeliveryServiceImpl`。

use async_trait::async_trait;
use sha1::Digest;
use std::sync::Weak;

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use crate::api::WxMaService;
use crate::api::g3_services::WxMaImmediateDeliveryService;
use crate::bean::WxMaBaseResponse;
use crate::bean::delivery::{
    AbnormalConfirmRequest, AbnormalConfirmResponse, AddOrderRequest, AddOrderResponse,
    BindAccountResponse, CancelOrderRequest, CancelOrderResponse, FollowWaybillRequest,
    FollowWaybillResponse, GetDeliveryListResponse, GetOrderRequest, GetOrderResponse,
    MockUpdateOrderRequest, MockUpdateOrderResponse, QueryFollowTraceRequest,
    QueryFollowTraceResponse, QueryWaybillTraceRequest, QueryWaybillTraceResponse,
    TraceWaybillRequest, TraceWaybillResponse, UpdateWaybillGoodsRequest,
};
use crate::enums::g3_urls::url_g3_shop::instant_delivery as delivery_url;

/// 顺丰同城响应码（对应 Java `SF_ERR_CODE`）。
const SF_ERR_CODE: &str = "resultcode";
/// 顺丰同城响应说明（对应 Java `SF_ERR_MSG`）。
const SF_ERR_MSG: &str = "resultmsg";
/// 成功响应状态码（对应 Java `SUCCESS_CODE`）。
const SUCCESS_CODE: i64 = 0;

/// 计算运力侧签名 delivery_sign（对应 Java `WxMaDeliveryBaseRequest.getDeliverySign()`）：
/// `SHA1(shopid + shop_order_id(有则拼) + appSecret)`，shopid/appSecret 为空时抛
/// RuntimeException（Rust 以 `WxErrorException::Runtime` 表达）。
fn build_delivery_sign(
    shop_id: &str,
    shop_order_id: &str,
    app_secret: &str,
) -> Result<String, WxErrorException> {
    if shop_id.is_empty() || app_secret.is_empty() {
        return Err(WxErrorException::Runtime(WxRuntimeError::new(
            "shopId or appSecret can not be empty",
        )));
    }
    let mut raw = String::from(shop_id);
    if !shop_order_id.is_empty() {
        raw.push_str(shop_order_id);
    }
    raw.push_str(app_secret);
    Ok(hex::encode(sha1::Sha1::digest(raw.as_bytes())))
}

/// 微信小程序即时配送服务实现。
pub struct WxMaImmediateDeliveryServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaImmediateDeliveryServiceImpl {
    /// 构建即时配送服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 解析响应（对应 Java `WxMaImmediateDeliveryServiceImpl.parse`）：
    /// 1) 响应为空抛 RuntimeException；2) `errcode` 存在且非 0 抛微信错误；
    /// 3) 运力方 `resultcode` 存在且非 0 抛 resultmsg；4) 反序列化为目标类型。
    fn parse<T>(response_content: &str) -> Result<T, WxErrorException>
    where
        T: serde::de::DeserializeOwned,
    {
        if response_content.is_empty() {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "the responseContent cannot be empty",
            )));
        }
        let json_object: serde_json::Value = serde_json::from_str(response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        // 是否为微信错误响应：当 errcode==0 或不存在时再看运力方 resultcode
        let element = json_object.get("errcode");
        if let Some(element) = element
            && !element.is_null()
            && SUCCESS_CODE != element.as_i64().unwrap_or_default()
        {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        // 是否为运力方错误响应
        let delivery = json_object.get(SF_ERR_CODE);
        if let Some(delivery) = delivery {
            if !delivery.is_null() && SUCCESS_CODE != delivery.as_i64().unwrap_or_default() {
                let msg = json_object
                    .get(SF_ERR_MSG)
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                return Err(WxErrorException::from_code(-99, msg));
            }
        }
        serde_json::from_str(response_content).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMaImmediateDeliveryService for WxMaImmediateDeliveryServiceImpl {
    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.getBindAccount`：
    /// POST `GET_BIND_ACCOUNT`（空对象 `{}`）后 `parse` 解析。
    async fn get_bind_account(&self) -> Result<BindAccountResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(&delivery_url::get_bind_account_url(config.as_ref()), "{}")
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.addOrder`：
    /// 先计算 delivery_sign（`request.getDeliverySign()`），POST `ADD_ORDER` 后 `parse` 解析。
    async fn add_order(
        &self,
        request: &AddOrderRequest,
    ) -> Result<AddOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let sign = build_delivery_sign(
            &request.shop_id,
            &request.shop_order_id,
            &request.app_secret,
        )?;
        let mut req = request.clone();
        req.delivery_sign = sign;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::add_order_url(config.as_ref()), &body)
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.getOrder`：
    /// 先计算 delivery_sign，POST `GET_ORDER` 后 `parse` 解析。
    async fn get_order(
        &self,
        request: &GetOrderRequest,
    ) -> Result<GetOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let sign = build_delivery_sign(
            &request.shop_id,
            &request.shop_order_id,
            &request.app_secret,
        )?;
        let mut req = request.clone();
        req.delivery_sign = sign;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::get_order_url(config.as_ref()), &body)
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.cancelOrder`：
    /// 先计算 delivery_sign，POST `CANCEL_ORDER` 后 `parse` 解析。
    async fn cancel_order(
        &self,
        request: &CancelOrderRequest,
    ) -> Result<CancelOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let sign = build_delivery_sign(
            &request.shop_id,
            &request.shop_order_id,
            &request.app_secret,
        )?;
        let mut req = request.clone();
        req.delivery_sign = sign;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::cancel_order_url(config.as_ref()), &body)
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.abnormalConfirm`：
    /// 先计算 delivery_sign，POST `ABNORMAL_CONFIRM` 后 `parse` 解析。
    async fn abnormal_confirm(
        &self,
        request: &AbnormalConfirmRequest,
    ) -> Result<AbnormalConfirmResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let sign = build_delivery_sign(
            &request.shop_id,
            &request.shop_order_id,
            &request.app_secret,
        )?;
        let mut req = request.clone();
        req.delivery_sign = sign;
        let body =
            serde_json::to_string(&req).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::abnormal_confirm_url(config.as_ref()), &body)
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.mockUpdateOrder`：
    /// POST `MOCK_UPDATE_ORDER`（序列化请求，沙盒环境）后 `parse` 解析。
    async fn mock_update_order(
        &self,
        request: &MockUpdateOrderRequest,
    ) -> Result<MockUpdateOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc
            .post(&delivery_url::mock_update_order_url(config.as_ref()), &body)
            .await?;
        Self::parse(&response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.traceWaybill`：
    /// POST `TRACE_WAYBILL_URL`（序列化请求），`fromJson` 解析；errcode==-1 抛微信错误。
    async fn trace_waybill(
        &self,
        request: &TraceWaybillRequest,
    ) -> Result<TraceWaybillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response_content = svc
            .post(&delivery_url::trace_waybill_url(config.as_ref()), &body)
            .await?;
        let response = TraceWaybillResponse::from_json(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.queryWaybillTrace`：
    /// POST `QUERY_WAYBILL_TRACE_URL`（序列化请求），`fromJson` 解析；errcode==-1 抛错。
    async fn query_waybill_trace(
        &self,
        request: &QueryWaybillTraceRequest,
    ) -> Result<QueryWaybillTraceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response_content = svc
            .post(
                &delivery_url::query_waybill_trace_url(config.as_ref()),
                &body,
            )
            .await?;
        let response = QueryWaybillTraceResponse::from_json(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.followWaybill`：
    /// POST `FOLLOW_WAYBILL_URL`（序列化请求），`fromJson` 解析；errcode==-1 抛错。
    async fn follow_waybill(
        &self,
        request: &FollowWaybillRequest,
    ) -> Result<FollowWaybillResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response_content = svc
            .post(&delivery_url::follow_waybill_url(config.as_ref()), &body)
            .await?;
        let response = FollowWaybillResponse::from_json(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.queryFollowTrace`：
    /// POST `QUERY_FOLLOW_TRACE_URL`（序列化请求），`fromJson` 解析；errcode==-1 抛错。
    async fn query_follow_trace(
        &self,
        request: &QueryFollowTraceRequest,
    ) -> Result<QueryFollowTraceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response_content = svc
            .post(
                &delivery_url::query_follow_trace_url(config.as_ref()),
                &body,
            )
            .await?;
        let response = QueryFollowTraceResponse::from_json(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.getDeliveryList`：
    /// POST `GET_DELIVERY_LIST_URL`（空对象 `{}`），`fromJson` 解析；errcode==-1 抛错。
    async fn get_delivery_list(&self) -> Result<GetDeliveryListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(&delivery_url::get_delivery_list_url(config.as_ref()), "{}")
            .await?;
        let response = GetDeliveryListResponse::from_json(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }

    /// 对应 Java `WxMaImmediateDeliveryServiceImpl.updateWaybillGoods`：
    /// POST `UPDATE_WAYBILL_GOODS_URL`（序列化请求），解析 `WxMaBaseResponse`；
    /// errcode==-1 抛微信错误。
    async fn update_waybill_goods(
        &self,
        request: &UpdateWaybillGoodsRequest,
    ) -> Result<WxMaBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body =
            serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response_content = svc
            .post(
                &delivery_url::update_waybill_goods_url(config.as_ref()),
                &body,
            )
            .await?;
        let response: WxMaBaseResponse = serde_json::from_str(&response_content)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        if response.errcode == -1 {
            let error = wx_rust_common::error::WxError::from_json_with_type(
                &response_content,
                Some(wx_rust_common::enums::WxType::MiniApp),
            );
            return Err(WxErrorException::from_code(
                error.error_code,
                error.error_msg.unwrap_or_default(),
            ));
        }
        Ok(response)
    }
}
