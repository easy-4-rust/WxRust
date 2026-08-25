//! WxChannelLimitedDiscountService（对应 Java `me.chanjar.weixin.channel.api.WxChannelLimitedDiscountService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::limit::{
    LimitTaskAddResponse, LimitTaskListResponse, LimitTaskParam, LimitTaskUpdateParam,
    LimitTaskUpdateResponse,
};

/// 限时抢购服务（对应 Java `WxChannelLimitedDiscountService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_limited_discount_service_impl` 的
/// `WxChannelLimitedDiscountServiceImpl`（Java `WxChannelLimitedDiscountServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelLimitedDiscountService: Send + Sync {
    /// 添加限时抢购任务（对应 Java `WxChannelLimitedDiscountService#addLimitTask(LimitTaskParam)`）。
    async fn add_limit_task(
        &self,
        param: LimitTaskParam,
    ) -> Result<LimitTaskAddResponse, WxErrorException>;

    /// 拉取限时抢购任务列表（对应 Java `WxChannelLimitedDiscountService#listLimitTask(Integer, String, Integer)`）。
    async fn list_limit_task(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<LimitTaskListResponse, WxErrorException>;

    /// 停止限时抢购任务（对应 Java `WxChannelLimitedDiscountService#stopLimitTask(String)`）。
    async fn stop_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除限时抢购任务（对应 Java `WxChannelLimitedDiscountService#deleteLimitTask(String)`）。
    async fn delete_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 更新限时抢购任务（对应 Java `WxChannelLimitedDiscountService#updateLimitTask(LimitTaskUpdateParam)`）。
    async fn update_limit_task(
        &self,
        param: LimitTaskUpdateParam,
    ) -> Result<LimitTaskUpdateResponse, WxErrorException>;
}
