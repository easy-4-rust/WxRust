//! WxMpMarketingService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMarketingService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::marketing::{
    WxMpAdLeadFilter, WxMpAdLeadResult, WxMpUserAction, WxMpUserActionSet,
};

/// 公众号MarketingService。
#[async_trait]
pub trait WxMpMarketingService: Send + Sync {
    async fn add_user_action_sets(
        &self,
        r#type: &str,
        name: &str,
        description: &str,
    ) -> Result<i64, WxErrorException>;

    async fn get_user_action_sets(
        &self,
        user_action_set_id: i64,
    ) -> Result<Vec<WxMpUserActionSet>, WxErrorException>;

    async fn add_user_action(&self, actions: &[WxMpUserAction]) -> Result<(), WxErrorException>;

    async fn get_ad_leads(
        &self,
        begin_date: &str,
        end_date: &str,
        filtering: &[WxMpAdLeadFilter],
        page: i32,
        page_size: i32,
    ) -> Result<WxMpAdLeadResult, WxErrorException>;
}
