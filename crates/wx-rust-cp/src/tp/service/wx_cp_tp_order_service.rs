//! 企业微信第三方应用应用版本付费订单服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpOrderService`：
//! 获取订单详情/订单列表
//! （https://developer.work.weixin.qq.com/document/15219）。

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::order::{WxCpTpOrderDetails, WxCpTpOrderListGetResult};

/// 企业微信第三方应用应用版本付费订单服务。
#[async_trait]
pub trait WxCpTpOrderService: Send + Sync {
    /// 获取订单详情（对应 Java `getOrder(String)`）。
    async fn get_order(&self, order_id: &str) -> Result<WxCpTpOrderDetails, WxErrorException>;

    /// 获取订单列表（对应 Java `getOrderList(Date, Date, Integer)`：
    /// testMode 0 正式模式，1 测试模式；Java `Date` 以
    /// `chrono::DateTime<Utc>` 表达，时间戳取秒）。
    async fn get_order_list(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        test_mode: Option<i32>,
    ) -> Result<WxCpTpOrderListGetResult, WxErrorException>;
}
