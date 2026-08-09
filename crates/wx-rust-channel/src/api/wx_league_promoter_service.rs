//! WxLeaguePromoterService（对应 Java `me.chanjar.weixin.channel.api.WxLeaguePromoterService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::promoter::{PromoterInfoResponse, PromoterListResponse};

/// 优选联盟 达人服务（对应 Java `WxLeaguePromoterService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_league_promoter_service_impl` 的
/// `WxLeaguePromoterServiceImpl`（Java `WxLeaguePromoterServiceImpl`）。
#[async_trait]
pub trait WxLeaguePromoterService: Send + Sync {
    /// 新增达人（对应 Java `WxLeaguePromoterService#addPromoter`，已废弃，建议 `add_promoter_v2`）。
    async fn add_promoter(
        &self,
        finder_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 编辑达人（对应 Java `WxLeaguePromoterService#updatePromoter`；
    /// `type`：1 取消邀请 / 2 结束合作）。
    async fn update_promoter(
        &self,
        finder_id: String,
        r#type: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除达人（对应 Java `WxLeaguePromoterService#deletePromoter`，已废弃）。
    async fn delete_promoter(
        &self,
        finder_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取达人详情信息（对应 Java `WxLeaguePromoterService#getPromoterInfo`，已废弃）。
    async fn get_promoter_info(
        &self,
        finder_id: String,
    ) -> Result<PromoterInfoResponse, WxErrorException>;

    /// 新增达人（对应 Java `WxLeaguePromoterService#addPromoterV2`；
    /// `promoter_id`：达人带货 id）。
    async fn add_promoter_v2(
        &self,
        promoter_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 编辑达人（对应 Java `WxLeaguePromoterService#updatePromoterV2`；
    /// `type`：1 取消邀请 / 2 结束合作）。
    async fn update_promoter_v2(
        &self,
        promoter_id: String,
        r#type: i32,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除达人（对应 Java `WxLeaguePromoterService#deletePromoterV2`）。
    async fn delete_promoter_v2(
        &self,
        promoter_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取达人详情信息（对应 Java `WxLeaguePromoterService#getPromoterInfoV2`）。
    async fn get_promoter_info_v2(
        &self,
        promoter_id: String,
    ) -> Result<PromoterInfoResponse, WxErrorException>;

    /// 获取达人列表（对应 Java `WxLeaguePromoterService#listPromoter`；
    /// `page_index` 从 1 开始，`page_size` 不超过 200）。
    async fn list_promoter(
        &self,
        page_index: Option<i32>,
        page_size: Option<i32>,
        status: Option<i32>,
    ) -> Result<PromoterListResponse, WxErrorException>;
}
