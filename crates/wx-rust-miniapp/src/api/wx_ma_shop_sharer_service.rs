//! 小程序交易组件-分享员服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopSharerService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::response::{
    WxMaShopSearchSharerResponse, WxMaShopSharerBindResponse, WxMaShopSharerDataSummaryResponse,
    WxMaShopSharerListResponse, WxMaShopSharerLiveOrderListResponse,
    WxMaShopSharerLiveSummaryListResponse, WxMaShopSharerUnbindResponse,
};

/// 小程序交易组件-分享员服务。
#[async_trait]
pub trait WxMaShopSharerService: Send + Sync {
    /// 绑定分享员（对应 Java `bindSharer(String[])`，用于批量邀请分享员）。
    async fn bind_sharer(
        &self,
        openids: &[String],
    ) -> Result<WxMaShopSharerBindResponse, WxErrorException>;

    /// 获取分享员的总带货数据（对应 Java `getSharerDataSummary(String)`）。
    async fn get_sharer_data_summary(
        &self,
        openid: &str,
    ) -> Result<WxMaShopSharerDataSummaryResponse, WxErrorException>;

    /// 获取已经绑定的分享员列表（对应 Java `getSharerList(Integer, Integer)`）。
    async fn get_sharer_list(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerListResponse, WxErrorException>;

    /// 获取分享员的直播间订单汇总（对应 Java `getSharerLiveOrderList(String, String, Integer, Integer)`）。
    async fn get_sharer_live_order_list(
        &self,
        openid: &str,
        live_export_id: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerLiveOrderListResponse, WxErrorException>;

    /// 获取分享员的直播间带货数据汇总（对应 Java `getSharerLiveSummaryList(String, Integer, Integer)`）。
    async fn get_sharer_live_summary_list(
        &self,
        openid: &str,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<WxMaShopSharerLiveSummaryListResponse, WxErrorException>;

    /// 查看分享员（对应 Java `searchSharer(String)`）。
    async fn search_sharer(
        &self,
        openid: &str,
    ) -> Result<WxMaShopSearchSharerResponse, WxErrorException>;

    /// 解绑分享员（对应 Java `unbindSharer(String[])`）。
    async fn unbind_sharer(
        &self,
        openids: &[String],
    ) -> Result<WxMaShopSharerUnbindResponse, WxErrorException>;
}
