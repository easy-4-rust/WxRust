//! 同城配送服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaIntracityServiceImpl`。
//! 两点适配（ADAPTED，注释标注）：
//! 1. Java 全部接口走 `postWithSignature`（API 签名通道，需 AES-GCM +
//!    RSA-PSS），门面 `WxMaService` 尚无该能力（见 wx_ma_service.rs 文档），
//!    本实现以普通 `post` 表达，签名通道待后续波次补齐；
//! 2. Java 以 `GsonBuilder().setFieldNamingPolicy(LOWER_CASE_WITH_UNDERSCORES)`
//!    序列化/反序列化（bean 无 @SerializedName），请求体线格式为 snake_case；
//!    生成的 Rust bean 为 camelCase 键，故经 `to_snake_case`/`from_snake_case`
//!    双向键转换桥接，保证线格式与 Java 逐字一致。
//! 3. Java 参数校验抛 `IllegalArgumentException`（unchecked），Rust 以
//!    `WxErrorException::Runtime` 承载（无对应 checked 变体）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::{WxErrorException, WxRuntimeError};

use crate::api::WxMaService;
use crate::api::g4_services::WxMaIntracityService;
use crate::bean::intractiy::{
    PayMode, WxMaAddOrderRequest, WxMaAddOrderResponse, WxMaCancelOrderResponse,
    WxMaGetPayModeResponse, WxMaOrder, WxMaPreAddOrderRequest, WxMaPreAddOrderResponse,
    WxMaQueryFlowRequest, WxMaStore, WxMaStoreBalance, WxMaStoreChargeRequest,
    WxMaStoreFlowResponse, WxMaStoreRefundRequest, WxMaTransCity,
};
use crate::enums::g4_urls::url_g4_ability::intracity as intracity_url;

/// 同城配送服务实现。
pub struct WxMaIntracityServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaIntracityServiceImpl {
    /// 构建同城配送服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 驼峰→snake_case（对应 Java Gson `LOWER_CASE_WITH_UNDERSCORES`：
    /// `ServiceTransPrefer` → `service_trans_prefer`，`wxStoreId` →
    /// `wx_store_id`）。
    fn camel_to_snake(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 4);
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_uppercase() {
                if i > 0 {
                    out.push('_');
                }
                out.push(c.to_ascii_lowercase());
            } else {
                out.push(c);
            }
        }
        out
    }

    /// snake_case→驼峰（`wx_store_id` → `wxStoreId`）。
    fn snake_to_camel(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut upper_next = false;
        for c in s.chars() {
            if c == '_' {
                upper_next = true;
            } else if upper_next {
                out.push(c.to_ascii_uppercase());
                upper_next = false;
            } else {
                out.push(c);
            }
        }
        out
    }

    /// 递归将对象键从 bean 的 camelCase 键转换为线格式 snake_case
    /// （对应 Java LOWER_CASE_WITH_UNDERSCORES 序列化）。
    fn to_snake_case(value: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let mut out = serde_json::Map::new();
                for (k, v) in map {
                    out.insert(Self::camel_to_snake(k), Self::to_snake_case(v));
                }
                serde_json::Value::Object(out)
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(Self::to_snake_case).collect())
            }
            other => other.clone(),
        }
    }

    /// 递归将响应键从线格式 snake_case 转换回 bean 的 camelCase 键。
    ///
    /// 为兼容 bean 中首个字母大写的字段（如 `ServiceTransPrefer`），对每个键
    /// 同时写入 `camelCase` 与首字母大写两种变体，serde 只取匹配的字段，
    /// 未匹配键被忽略。
    fn from_snake_case(value: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let mut out = serde_json::Map::new();
                for (k, v) in map {
                    let camel = Self::snake_to_camel(k);
                    let mut capital = camel.clone();
                    if let Some(first) = capital.chars().next() {
                        capital = first.to_ascii_uppercase().to_string() + &capital[1..];
                    }
                    let converted = Self::from_snake_case(v);
                    out.insert(camel.clone(), converted.clone());
                    if capital != camel {
                        out.insert(capital, converted);
                    }
                }
                serde_json::Value::Object(out)
            }
            serde_json::Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(Self::from_snake_case).collect())
            }
            other => other.clone(),
        }
    }

    /// 以 snake_case 序列化请求对象（对应 Java gson LOWER_CASE_WITH_UNDERSCORES）。
    fn to_snake_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        let value = serde_json::to_value(request).map_err(WxErrorException::from)?;
        Ok(Self::to_snake_case(&value).to_string())
    }

    /// POST 并解析响应（响应经 snake_case→bean 键转换，对应 Java gson
    /// LOWER_CASE_WITH_UNDERSCORES 反序列化）。
    async fn post_snake<T>(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_body).await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        let value = Self::from_snake_case(&value);
        serde_json::from_value(value).map_err(WxErrorException::from)
    }

    /// 校验响应 errcode（对应 Java `checkStringResponse`）。
    async fn check_string_response(
        svc: &dyn WxMaService,
        url: &str,
        post_body: &str,
    ) -> Result<(), WxErrorException> {
        svc.post(url, post_body).await?;
        Ok(())
    }

    /// 私有查询门店（对应 Java `WxMaIntracityServiceImpl.queryStore`）。
    async fn query_store(
        &self,
        wx_store_id: Option<&str>,
        out_store_id: Option<&str>,
    ) -> Result<Vec<WxMaStore>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let mut map = serde_json::Map::new();
        if let Some(wx_store_id) = wx_store_id {
            map.insert("wx_store_id".to_string(), serde_json::json!(wx_store_id));
        } else if let Some(out_store_id) = out_store_id {
            map.insert("out_store_id".to_string(), serde_json::json!(out_store_id));
        }
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &intracity_url::query_store_url(config.as_ref()),
                &serde_json::Value::Object(map).to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        let json = Self::from_snake_case(&json);
        let store_list = json
            .get("store_list")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        serde_json::from_value(store_list).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaIntracityService for WxMaIntracityServiceImpl {
    /// 申请开通门店权限（对应 Java `WxMaIntracityServiceImpl.apply`，
    /// 请求体 `{}`）。
    async fn apply(&self) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::check_string_response(
            svc.as_ref(),
            &intracity_url::apply_url(config.as_ref()),
            "{}",
        )
        .await
    }

    /// 创建门店（对应 Java `WxMaIntracityServiceImpl.createStore`，返回
    /// `wx_store_id`）。
    ///
    /// 校验：outStoreId 不能为空；wxStoreId 必须为空（Java
    /// `IllegalArgumentException`，Rust 以 `Runtime` 承载）。
    async fn create_store(&self, store: &WxMaStore) -> Result<String, WxErrorException> {
        if store.out_store_id.is_empty() {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "创建门店时outStoreId不能为空",
            )));
        }
        if !store.wx_store_id.is_empty() {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "创建门店时wxStoreId只能是null",
            )));
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &intracity_url::create_store_url(config.as_ref()),
                &Self::to_snake_json(store)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        json.get("wx_store_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "wx_store_id 字段缺失"))
    }

    /// 更新门店（对应 Java `WxMaIntracityServiceImpl.updateStore`）。
    ///
    /// 请求体 `{"keys":{wx_store_id|out_store_id}, "content":{可更新字段}}`；
    /// 校验：wxStoreId 与 outStoreId 至少一个非空（Java
    /// `IllegalArgumentException`）。
    async fn update_store(&self, store: &WxMaStore) -> Result<(), WxErrorException> {
        if store.wx_store_id.is_empty() && store.out_store_id.is_empty() {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "更新门店时wxStoreId 或 outStoreId 至少要有一个不为null",
            )));
        }
        let mut keys = serde_json::Map::new();
        if !store.wx_store_id.is_empty() {
            keys.insert(
                "wx_store_id".to_string(),
                serde_json::json!(store.wx_store_id),
            );
        } else {
            keys.insert(
                "out_store_id".to_string(),
                serde_json::json!(store.out_store_id),
            );
        }
        let mut content = serde_json::Map::new();
        if !store.store_name.is_empty() {
            content.insert(
                "store_name".to_string(),
                serde_json::json!(store.store_name),
            );
        }
        if store.order_pattern == 1 || store.order_pattern == 2 {
            content.insert(
                "order_pattern".to_string(),
                serde_json::json!(store.order_pattern),
            );
        }
        if !store.service_trans_prefer.is_empty() {
            content.insert(
                "service_trans_prefer".to_string(),
                serde_json::json!(store.service_trans_prefer),
            );
        }
        if !store.address_info.province.is_empty()
            || !store.address_info.city.is_empty()
            || !store.address_info.area.is_empty()
            || !store.address_info.street.is_empty()
            || !store.address_info.house.is_empty()
            || !store.address_info.phone.is_empty()
        {
            content.insert(
                "address_info".to_string(),
                Self::to_snake_case(&serde_json::json!(store.address_info)),
            );
        }
        let post_body = serde_json::json!({
            "keys": serde_json::Value::Object(keys),
            "content": serde_json::Value::Object(content),
        })
        .to_string();
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::check_string_response(
            svc.as_ref(),
            &intracity_url::update_store_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 查询门店（列出所有门店，对应 Java `WxMaIntracityServiceImpl.listAllStores`）。
    async fn list_all_stores(&self) -> Result<Vec<WxMaStore>, WxErrorException> {
        self.query_store(None, None).await
    }

    /// 根据 `wx_store_id` 查询门店（对应 Java
    /// `WxMaIntracityServiceImpl.queryStoreByWxStoreId`，列表为空返回 `None`）。
    async fn query_store_by_wx_store_id(
        &self,
        wx_store_id: &str,
    ) -> Result<Option<WxMaStore>, WxErrorException> {
        let list = self.query_store(Some(wx_store_id), None).await?;
        Ok(list.into_iter().next())
    }

    /// 根据 `out_store_id` 查询门店（对应 Java
    /// `WxMaIntracityServiceImpl.queryStoreByOutStoreId`）。
    async fn query_store_by_out_store_id(
        &self,
        out_store_id: &str,
    ) -> Result<Vec<WxMaStore>, WxErrorException> {
        self.query_store(None, Some(out_store_id)).await
    }

    /// 门店运费充值（对应 Java `WxMaIntracityServiceImpl.storeCharge`，
    /// 返回 `payurl`）。
    async fn store_charge(
        &self,
        request: &WxMaStoreChargeRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &intracity_url::store_charge_url(config.as_ref()),
                &Self::to_snake_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        json.get("payurl")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "payurl 字段缺失"))
    }

    /// 门店运费退款（对应 Java `WxMaIntracityServiceImpl.storeRefund`，
    /// 返回 `refund_amount`）。
    async fn store_refund(
        &self,
        request: &WxMaStoreRefundRequest,
    ) -> Result<i32, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &intracity_url::store_refund_url(config.as_ref()),
                &Self::to_snake_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        json.get("refund_amount")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32)
            .ok_or_else(|| WxErrorException::from_code(-99, "refund_amount 字段缺失"))
    }

    /// 门店运费流水查询（对应 Java `WxMaIntracityServiceImpl.queryFlow`）。
    ///
    /// 校验：wxStoreId 非空、flowType 为 1/2/3 之一（Java
    /// `IllegalArgumentException`）。Java 按 flowType 动态选择
    /// Charge/Refund/Consume 泛型实例；Rust bean `WxMaStoreFlowResponse`
    /// 的 `flow_list` 以 `serde_json::Value` 承载各类流水记录（ADAPTED）。
    async fn query_flow(
        &self,
        request: &WxMaQueryFlowRequest,
    ) -> Result<WxMaStoreFlowResponse, WxErrorException> {
        if request.wx_store_id.is_empty() {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "查询请求 wxStoreId 不可为空",
            )));
        }
        if !(1..=3).contains(&request.flow_type) {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "查询请求 flowType 不正确，只能是1、2、3之一",
            )));
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::query_flow_url(config.as_ref()),
            &Self::to_snake_json(request)?,
        )
        .await
    }

    /// 查询门店余额（对应 Java `WxMaIntracityServiceImpl.balanceQuery`）。
    ///
    /// 校验：payMode 为 STORE 或空时必须传 wxStoreId（Java
    /// `IllegalArgumentException`）。
    async fn balance_query(
        &self,
        wx_store_id: Option<&str>,
        service_trans_id: Option<&str>,
        pay_mode: Option<PayMode>,
    ) -> Result<WxMaStoreBalance, WxErrorException> {
        if wx_store_id.is_none() && (pay_mode.is_none() || pay_mode == Some(PayMode::Store)) {
            return Err(WxErrorException::Runtime(WxRuntimeError::new(
                "payMode是PAY_MODE_STORE或null时，必须传递wxStoreId",
            )));
        }
        let mut map = serde_json::Map::new();
        if let Some(wx_store_id) = wx_store_id {
            map.insert("wx_store_id".to_string(), serde_json::json!(wx_store_id));
        }
        if let Some(service_trans_id) = service_trans_id {
            map.insert(
                "service_trans_id".to_string(),
                serde_json::json!(service_trans_id),
            );
        }
        if let Some(pay_mode) = pay_mode {
            map.insert("pay_mode".to_string(), serde_json::json!(pay_mode));
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::balance_query_url(config.as_ref()),
            &serde_json::Value::Object(map).to_string(),
        )
        .await
    }

    /// 设置扣费主体（对应 Java `WxMaIntracityServiceImpl.setPayMode`，
    /// 请求体含 `appid`）。
    async fn set_pay_mode(&self, pay_mode: PayMode) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let appid = config.app_id();
        let post_body = serde_json::json!({
            "pay_mode": pay_mode,
            "appid": appid,
        })
        .to_string();
        let config = svc.wx_ma_config();
        Self::check_string_response(
            svc.as_ref(),
            &intracity_url::set_pay_mode_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 查询扣费主体（对应 Java `WxMaIntracityServiceImpl.getPayMode`，
    /// 请求体含 `appid`）。
    async fn get_pay_mode(&self) -> Result<WxMaGetPayModeResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let appid = config.app_id();
        let post_body = serde_json::json!({ "appid": appid }).to_string();
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::get_pay_mode_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 查询运费（对应 Java `WxMaIntracityServiceImpl.preAddOrder`）。
    async fn pre_add_order(
        &self,
        request: &WxMaPreAddOrderRequest,
    ) -> Result<WxMaPreAddOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::pre_add_order_url(config.as_ref()),
            &Self::to_snake_json(request)?,
        )
        .await
    }

    /// 创建配送单（对应 Java `WxMaIntracityServiceImpl.addOrder`）。
    async fn add_order(
        &self,
        request: &WxMaAddOrderRequest,
    ) -> Result<WxMaAddOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::add_order_url(config.as_ref()),
            &Self::to_snake_json(request)?,
        )
        .await
    }

    /// 根据微信订单号查询配送单（对应 Java
    /// `WxMaIntracityServiceImpl.queryOrderByWxOrderId`）。
    async fn query_order_by_wx_order_id(
        &self,
        wx_order_id: &str,
    ) -> Result<WxMaOrder, WxErrorException> {
        let post_body = serde_json::json!({ "wx_order_id": wx_order_id }).to_string();
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::query_order_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 依据商户订单号查询配送单（对应 Java
    /// `WxMaIntracityServiceImpl.queryOrderByStoreOrderId`）。
    async fn query_order_by_store_order_id(
        &self,
        wx_store_id: &str,
        store_order_id: &str,
    ) -> Result<WxMaOrder, WxErrorException> {
        let post_body = serde_json::json!({
            "wx_store_id": wx_store_id,
            "store_order_id": store_order_id,
        })
        .to_string();
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::query_order_url(config.as_ref()),
            &post_body,
        )
        .await
    }

    /// 依据微信订单号取消配送单（对应 Java
    /// `WxMaIntracityServiceImpl.cancelOrderByWxOrderId`）。
    async fn cancel_order_by_wx_order_id(
        &self,
        wx_order_id: &str,
        cancel_reason_id: i32,
        cancel_reason: Option<&str>,
    ) -> Result<WxMaCancelOrderResponse, WxErrorException> {
        let mut map = serde_json::Map::new();
        map.insert("wx_order_id".to_string(), serde_json::json!(wx_order_id));
        map.insert(
            "cancel_reason_id".to_string(),
            serde_json::json!(cancel_reason_id),
        );
        if let Some(cancel_reason) = cancel_reason {
            map.insert(
                "cancel_reason".to_string(),
                serde_json::json!(cancel_reason),
            );
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::cancel_order_url(config.as_ref()),
            &serde_json::Value::Object(map).to_string(),
        )
        .await
    }

    /// 依据商户订单号取消配送单（对应 Java
    /// `WxMaIntracityServiceImpl.cancelOrderByStoreOrderId`）。
    async fn cancel_order_by_store_order_id(
        &self,
        wx_store_id: &str,
        store_order_id: &str,
        cancel_reason_id: i32,
        cancel_reason: Option<&str>,
    ) -> Result<WxMaCancelOrderResponse, WxErrorException> {
        let mut map = serde_json::Map::new();
        map.insert("wx_store_id".to_string(), serde_json::json!(wx_store_id));
        map.insert(
            "store_order_id".to_string(),
            serde_json::json!(store_order_id),
        );
        map.insert(
            "cancel_reason_id".to_string(),
            serde_json::json!(cancel_reason_id),
        );
        if let Some(cancel_reason) = cancel_reason {
            map.insert(
                "cancel_reason".to_string(),
                serde_json::json!(cancel_reason),
            );
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        Self::post_snake(
            svc.as_ref(),
            &intracity_url::cancel_order_url(config.as_ref()),
            &serde_json::Value::Object(map).to_string(),
        )
        .await
    }

    /// 查询支持同城配送的城市（对应 Java
    /// `WxMaIntracityServiceImpl.getCity`，解析 `support_list`；
    /// `service_trans_id` 为空时返回所有）。
    async fn get_city(
        &self,
        service_trans_id: Option<&str>,
    ) -> Result<Vec<WxMaTransCity>, WxErrorException> {
        let mut map = serde_json::Map::new();
        if let Some(service_trans_id) = service_trans_id {
            map.insert(
                "service_trans_id".to_string(),
                serde_json::json!(service_trans_id),
            );
        }
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &intracity_url::get_city_url(config.as_ref()),
                &serde_json::Value::Object(map).to_string(),
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(WxErrorException::from)?;
        let json = Self::from_snake_case(&json);
        let support_list = json
            .get("support_list")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        serde_json::from_value(support_list).map_err(WxErrorException::from)
    }
}
