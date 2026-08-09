//! 企业微信直播服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpLivingService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    LivingIdResult, WxCpLivingCreateRequest, WxCpLivingInfo, WxCpLivingModifyRequest,
    WxCpLivingResult, WxCpLivingShareInfo, WxCpWatchStat,
};

/// 企业微信直播服务。
#[async_trait]
pub trait WxCpLivingService: Send + Sync {
    /// 获取微信观看直播凭证（对应 Java
    /// `WxCpLivingService.getLivingCode(String, String)`）。
    async fn get_living_code(
        &self,
        open_id: &str,
        living_id: &str,
    ) -> Result<String, WxErrorException>;

    /// 获取直播详情（对应 Java
    /// `WxCpLivingService.getLivingInfo(String)`）。
    async fn get_living_info(&self, living_id: &str) -> Result<WxCpLivingInfo, WxErrorException>;

    /// 获取直播观看明细（对应 Java
    /// `WxCpLivingService.getWatchStat(String, String)`；`nextKey` 初次
    /// 调用可以填 `"0"`）。
    async fn get_watch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpWatchStat, WxErrorException>;

    /// 获取成员直播 ID 列表（对应 Java
    /// `WxCpLivingService.getUserAllLivingId(String, String, Integer)`）。
    async fn get_user_all_living_id(
        &self,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<LivingIdResult, WxErrorException>;

    /// 获取跳转小程序商城的直播观众信息（对应 Java
    /// `WxCpLivingService.getLivingShareInfo(String)`）。
    async fn get_living_share_info(
        &self,
        ww_share_code: &str,
    ) -> Result<WxCpLivingShareInfo, WxErrorException>;

    /// 创建预约直播（对应 Java
    /// `WxCpLivingService.livingCreate(WxCpLivingCreateRequest)`，
    /// 返回直播 id）。
    async fn living_create(
        &self,
        request: &WxCpLivingCreateRequest,
    ) -> Result<String, WxErrorException>;

    /// 修改预约直播（对应 Java
    /// `WxCpLivingService.livingModify(WxCpLivingModifyRequest)`）。
    async fn living_modify(
        &self,
        request: &WxCpLivingModifyRequest,
    ) -> Result<WxCpLivingResult, WxErrorException>;

    /// 取消预约直播（对应 Java
    /// `WxCpLivingService.livingCancel(String)`）。
    async fn living_cancel(&self, living_id: &str) -> Result<WxCpLivingResult, WxErrorException>;

    /// 删除直播回放（对应 Java
    /// `WxCpLivingService.deleteReplayData(String)`）。
    async fn delete_replay_data(
        &self,
        living_id: &str,
    ) -> Result<WxCpLivingResult, WxErrorException>;
}
