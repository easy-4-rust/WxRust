//! WxStoreSharerService（对应 Java `com.binarywang.wxjava.store.api.WxStoreSharerService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::sharer::{
    SharerBindResponse, SharerInfoResponse, SharerOrderParam, SharerOrderResponse,
    SharerSearchResponse, SharerUnbindResponse,
};

/// 分享员服务（对应 Java `WxStoreSharerService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_store_sharer_service_impl` 的
/// `WxStoreSharerServiceImpl`（Java `WxStoreSharerServiceImpl`）。
#[async_trait::async_trait]
pub trait WxStoreSharerService: Send + Sync {
    /// 邀请分享员（对应 Java `WxStoreSharerService#bindSharer(String)`）。
    ///
    /// # 参数
    /// - `username`：邀请的用户微信号
    async fn bind_sharer(&self, username: String) -> Result<SharerBindResponse, WxErrorException>;

    /// 获取绑定的分享员（对应 Java
    /// `WxStoreSharerService#searchSharer(String, String)`）。
    ///
    /// # 参数
    /// - `openid`：分享员 openid
    /// - `username`：分享员微信号（二选一）
    async fn search_sharer(
        &self,
        openid: String,
        username: String,
    ) -> Result<SharerSearchResponse, WxErrorException>;

    /// 获取绑定的分享员列表（对应 Java
    /// `WxStoreSharerService#listSharer(Integer, Integer, Integer)`）。
    ///
    /// # 参数
    /// - `page`：分页参数，页数
    /// - `page_size`：分页参数，每页分享员数（不超过 100）
    /// - `sharer_type`：分享员类型
    async fn list_sharer(
        &self,
        page: Option<i32>,
        page_size: Option<i32>,
        sharer_type: Option<i32>,
    ) -> Result<SharerInfoResponse, WxErrorException>;

    /// 获取分享员订单列表（对应 Java
    /// `WxStoreSharerService#listSharerOrder(SharerOrderParam)`）。
    async fn list_sharer_order(
        &self,
        param: SharerOrderParam,
    ) -> Result<SharerOrderResponse, WxErrorException>;

    /// 解绑分享员（对应 Java `WxStoreSharerService#unbindSharer(List<String>)`）。
    ///
    /// # 参数
    /// - `open_ids`：openid 列表
    async fn unbind_sharer(
        &self,
        open_ids: Vec<String>,
    ) -> Result<SharerUnbindResponse, WxErrorException>;
}
