//! 企业微信第三方应用应用版本付费订单服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpOrderServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面；Java `Date` 以
//! `chrono::DateTime<Utc>` 表达，时间戳取秒（Java
//! `startTime.getTime() / 1000`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::order::{WxCpTpOrderDetails, WxCpTpOrderListGetResult};
use crate::enums::url_tp;
use crate::tp::service::{WxCpTpOrderService, WxCpTpService};

/// 企业微信第三方应用应用版本付费订单服务实现。
pub struct WxCpTpOrderServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpOrderServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }
}

#[async_trait]
impl WxCpTpOrderService for WxCpTpOrderServiceImpl {
    async fn get_order(&self, order_id: &str) -> Result<WxCpTpOrderDetails, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tp::GET_ORDER);
        let body = serde_json::json!({ "orderid": order_id }).to_string();
        let result = service.post(&url, &body).await?;
        WxCpTpOrderDetails::from_json(&result).map_err(WxErrorException::Serde)
    }

    async fn get_order_list(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        test_mode: Option<i32>,
    ) -> Result<WxCpTpOrderListGetResult, WxErrorException> {
        let service = self.service()?;
        let config = service.wx_cp_tp_config_storage();
        let url = config.api_url(url_tp::GET_ORDER_LIST);
        let body = serde_json::json!({
            "start_time": start_time.timestamp(),
            "end_time": end_time.timestamp(),
            "test_mode": test_mode,
        })
        .to_string();
        let result = service.post(&url, &body).await?;
        WxCpTpOrderListGetResult::from_json(&result).map_err(WxErrorException::Serde)
    }
}
