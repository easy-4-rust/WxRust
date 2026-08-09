//! WxMpShakeService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpShakeService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::shake::{
    WxMpShakeAroundDeviceBindPageQuery, WxMpShakeAroundPageAddQuery, WxMpShakeAroundPageAddResult,
    WxMpShakeAroundRelationSearchQuery, WxMpShakeAroundRelationSearchResult,
};
use crate::bean::{WxMpShakeInfoResult, WxMpShakeQuery};
use crate::enums::wx_mp_api_url::shake;

/// 公众号ShakeService。
#[async_trait]
pub trait WxMpShakeService: Send + Sync {
    async fn get_shake_info(
        &self,
        query: &WxMpShakeQuery,
    ) -> Result<WxMpShakeInfoResult, WxErrorException>;

    async fn page_add(
        &self,
        query: &WxMpShakeAroundPageAddQuery,
    ) -> Result<WxMpShakeAroundPageAddResult, WxErrorException>;

    async fn device_bind_page_query(
        &self,
        query: &WxMpShakeAroundDeviceBindPageQuery,
    ) -> Result<bool, WxErrorException>;

    async fn relation_search(
        &self,
        query: &WxMpShakeAroundRelationSearchQuery,
    ) -> Result<WxMpShakeAroundRelationSearchResult, WxErrorException>;
}
