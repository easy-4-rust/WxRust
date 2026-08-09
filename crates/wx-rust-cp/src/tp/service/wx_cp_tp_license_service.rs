//! 企业微信第三方应用接口许可服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.WxCpTpLicenseService`：
//! 服务商接口调用许可相关接口
//! （https://developer.work.weixin.qq.com/document/path/95652）。

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::WxCpBaseResp;
use crate::bean::license::account::{
    WxCpTpLicenseActiveInfoByUserResp, WxCpTpLicenseBatchActiveResultResp,
    WxCpTpLicenseBatchCodeInfoResp, WxCpTpLicenseBatchTransferResp, WxCpTpLicenseCodeInfoResp,
    WxCpTpLicenseCorpAccountListResp,
};
use crate::bean::license::order::{
    WxCpTpLicenseCreateOrderResp, WxCpTpLicenseNewOrderRequest, WxCpTpLicenseOrderAccountListResp,
    WxCpTpLicenseOrderInfoResp, WxCpTpLicenseOrderListResp, WxCpTpLicenseRenewOrderJobRequest,
    WxCpTpLicenseRenewOrderJobResp, WxCpTpLicenseRenewOrderRequest,
};
use crate::bean::license::{WxCpTpLicenseActiveAccount, WxCpTpLicenseTransfer};

/// 企业微信第三方应用接口许可服务。
#[async_trait]
pub trait WxCpTpLicenseService: Send + Sync {
    /// 下单购买账号（对应 Java
    /// `createNewOrder(WxCpTpLicenseNewOrderRequest)`，返回订单 ID）。
    async fn create_new_order(
        &self,
        license_new_order_request: &WxCpTpLicenseNewOrderRequest,
    ) -> Result<WxCpTpLicenseCreateOrderResp, WxErrorException>;

    /// 创建下单续期账号任务（对应 Java
    /// `createRenewOrderJob(WxCpTpLicenseRenewOrderJobRequest)`，返回
    /// JobId）。
    async fn create_renew_order_job(
        &self,
        license_renew_order_job_request: &WxCpTpLicenseRenewOrderJobRequest,
    ) -> Result<WxCpTpLicenseRenewOrderJobResp, WxErrorException>;

    /// 提交续期订单（对应 Java
    /// `submitRenewOrder(WxCpTpLicenseRenewOrderRequest)`，返回订单 ID）。
    async fn submit_renew_order(
        &self,
        license_renew_order_request: &WxCpTpLicenseRenewOrderRequest,
    ) -> Result<WxCpTpLicenseCreateOrderResp, WxErrorException>;

    /// 获取订单列表（对应 Java `getOrderList(String, Date, Date, String,
    /// int)`；startTime/endTime 须同时指定且不超过 31 天；limit 最大
    /// 1000 默认 500）。
    async fn get_order_list(
        &self,
        corp_id: &str,
        start_time: Option<DateTime<Utc>>,
        end_time: Option<DateTime<Utc>>,
        cursor: &str,
        limit: i32,
    ) -> Result<WxCpTpLicenseOrderListResp, WxErrorException>;

    /// 获取订单详情（对应 Java `getOrderInfo(String)`）。
    async fn get_order_info(
        &self,
        order_id: &str,
    ) -> Result<WxCpTpLicenseOrderInfoResp, WxErrorException>;

    /// 查询指定订单下的平台能力服务账号列表（对应 Java
    /// `getOrderAccountList(String, int, String)`）。
    async fn get_order_account_list(
        &self,
        order_id: &str,
        limit: i32,
        cursor: &str,
    ) -> Result<WxCpTpLicenseOrderAccountListResp, WxErrorException>;

    /// 激活账号（对应 Java `activeCode(String, String, String)`）。
    async fn active_code(
        &self,
        code: &str,
        corp_id: &str,
        user_id: &str,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 批量激活账号（对应 Java `batchActiveCode(String,
    /// List<WxCpTpLicenseActiveAccount>)`，单次激活不超过 1000）。
    async fn batch_active_code(
        &self,
        corp_id: &str,
        active_account_list: &[WxCpTpLicenseActiveAccount],
    ) -> Result<WxCpTpLicenseBatchActiveResultResp, WxErrorException>;

    /// 获取激活码详情（对应 Java `getActiveInfoByCode(String, String)`）。
    async fn get_active_info_by_code(
        &self,
        code: &str,
        corp_id: &str,
    ) -> Result<WxCpTpLicenseCodeInfoResp, WxErrorException>;

    /// 批量获取激活码详情（对应 Java `batchGetActiveInfoByCode(Collection,
    /// String)`）。
    async fn batch_get_active_info_by_code(
        &self,
        codes: &[String],
        corp_id: &str,
    ) -> Result<WxCpTpLicenseBatchCodeInfoResp, WxErrorException>;

    /// 获取企业的账号列表（对应 Java `getCorpAccountList(String, int,
    /// String)`）。
    async fn get_corp_account_list(
        &self,
        corp_id: &str,
        limit: i32,
        cursor: &str,
    ) -> Result<WxCpTpLicenseCorpAccountListResp, WxErrorException>;

    /// 获取成员的激活详情（对应 Java `getActiveInfoByUser(String, String)`）。
    async fn get_active_info_by_user(
        &self,
        corp_id: &str,
        user_id: &str,
    ) -> Result<WxCpTpLicenseActiveInfoByUserResp, WxErrorException>;

    /// 账号继承（对应 Java `batchTransferLicense(String,
    /// List<WxCpTpLicenseTransfer>)`）。
    async fn batch_transfer_license(
        &self,
        corp_id: &str,
        transfer_list: &[WxCpTpLicenseTransfer],
    ) -> Result<WxCpTpLicenseBatchTransferResp, WxErrorException>;
}
