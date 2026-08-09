//! 小程序交易组件-售后服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopAfterSaleService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::request::{
    WxMaShopAcceptReturnRequest, WxMaShopAfterSaleAddRequest, WxMaShopAfterSaleGetRequest,
    WxMaShopAfterSaleListRequest, WxMaShopAfterSaleUpdateRequest,
    WxMaShopAfterSaleUploadReturnInfoRequest, WxMaShopEcAfterSaleGetRequest,
    WxMaShopEcAfterSaleUpdateRequest, WxMaShopUploadCerficatesRequest,
};
use crate::bean::shop::response::{
    WxMaShopAfterSaleAddResponse, WxMaShopAfterSaleGetResponse, WxMaShopAfterSaleListResponse,
    WxMaShopBaseResponse, WxMaShopEcAfterSaleGetResponse,
};

/// 小程序交易组件-售后服务。
#[async_trait]
pub trait WxMaShopAfterSaleService: Send + Sync {
    /// 创建售后（对应 Java `add(WxMaShopAfterSaleAddRequest)`）。
    async fn add(
        &self,
        request: &WxMaShopAfterSaleAddRequest,
    ) -> Result<WxMaShopAfterSaleAddResponse, WxErrorException>;

    /// 获取订单下售后单（对应 Java `get(WxMaShopAfterSaleGetRequest)`）。
    async fn get(
        &self,
        request: &WxMaShopAfterSaleGetRequest,
    ) -> Result<WxMaShopAfterSaleGetResponse, WxErrorException>;

    /// 获取售后单详情（EC 版，对应 Java `get(WxMaShopEcAfterSaleGetRequest)`）。
    async fn get_ec(
        &self,
        request: &WxMaShopEcAfterSaleGetRequest,
    ) -> Result<WxMaShopEcAfterSaleGetResponse, WxErrorException>;

    /// 更新售后（对应 Java `update(WxMaShopAfterSaleUpdateRequest)`）。
    async fn update(
        &self,
        request: &WxMaShopAfterSaleUpdateRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 更新售后（EC 版，对应 Java `update(WxMaShopEcAfterSaleUpdateRequest)`）。
    async fn update_ec(
        &self,
        request: &WxMaShopEcAfterSaleUpdateRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 用户取消售后申请（对应 Java `cancel(String, Long, String)`）。
    ///
    /// `out_after_sale_id` 商家自定义售后 ID；`after_sale_id` 微信侧售后 ID
    /// （与 out_aftersale_id 二选一）；`open_id` 用户 openid。
    async fn cancel(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
        open_id: &str,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 用户上传退货物流（对应 Java `uploadReturnInfo(WxMaShopAfterSaleUploadReturnInfoRequest)`）。
    async fn upload_return_info(
        &self,
        request: &WxMaShopAfterSaleUploadReturnInfoRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商家同意退款（对应 Java `acceptRefund(String, Long)`）。
    async fn accept_refund(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商家同意退货（对应 Java `acceptReturn(WxMaShopAcceptReturnRequest)`）。
    async fn accept_return(
        &self,
        request: &WxMaShopAcceptReturnRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商家拒绝售后（对应 Java `reject(String, Long)`）。
    async fn reject(
        &self,
        out_after_sale_id: Option<&str>,
        after_sale_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商家上传退款凭证（对应 Java `uploadCertificates(WxMaShopUploadCerficatesRequest)`）。
    async fn upload_certificates(
        &self,
        request: &WxMaShopUploadCerficatesRequest,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商家更新订单售后期（对应 Java `updateDeadline(String, Long, String, Long)`）。
    async fn update_deadline(
        &self,
        out_order_id: Option<&str>,
        order_id: Option<i64>,
        openid: &str,
        after_sale_deadline: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取售后单列表（对应 Java `list(WxMaShopAfterSaleListRequest)`）。
    async fn list(
        &self,
        request: &WxMaShopAfterSaleListRequest,
    ) -> Result<WxMaShopAfterSaleListResponse, WxErrorException>;
}
