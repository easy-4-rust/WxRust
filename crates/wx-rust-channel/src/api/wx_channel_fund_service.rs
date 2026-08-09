//! WxChannelFundService（对应 Java `me.chanjar.weixin.channel.api.WxChannelFundService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::fund::bank::{
    BankCityResponse, BankInfoResponse, BankListResponse, BankProvinceResponse, BranchInfoResponse,
};
use crate::bean::fund::qrcode::{QrCheckResponse, QrCodeResponse};
use crate::bean::fund::{
    AccountInfo, AccountInfoResponse, BalanceInfoResponse, FlowListResponse, FundsFlowResponse,
    FundsListParam, WithdrawDetailResponse, WithdrawListResponse, WithdrawSubmitResponse,
};

/// 资金相关服务（对应 Java `WxChannelFundService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_channel_fund_service_impl` 的
/// `WxChannelFundServiceImpl`（Java `WxChannelFundServiceImpl`）。
#[async_trait]
pub trait WxChannelFundService: Send + Sync {
    /// 获取账户余额（对应 Java `WxChannelFundService#getBalance`）。
    async fn get_balance(&self) -> Result<BalanceInfoResponse, WxErrorException>;

    /// 获取结算账户（对应 Java `WxChannelFundService#getBankAccount`）。
    async fn get_bank_account(&self) -> Result<AccountInfoResponse, WxErrorException>;

    /// 获取资金流水详情（对应 Java `WxChannelFundService#getFundsFlowDetail(String)`）。
    ///
    /// # 参数
    /// - `flow_id`：资金流水号
    async fn get_funds_flow_detail(
        &self,
        flow_id: String,
    ) -> Result<FundsFlowResponse, WxErrorException>;

    /// 获取资金流水列表（对应 Java `WxChannelFundService#listFundsFlow(FundsListParam)`）。
    async fn list_funds_flow(
        &self,
        param: FundsListParam,
    ) -> Result<FlowListResponse, WxErrorException>;

    /// 获取提现记录（对应 Java `WxChannelFundService#getWithdrawDetail(String)`）。
    ///
    /// # 参数
    /// - `withdraw_id`：提现单号
    async fn get_withdraw_detail(
        &self,
        withdraw_id: String,
    ) -> Result<WithdrawDetailResponse, WxErrorException>;

    /// 获取提现记录列表（对应 Java
    /// `WxChannelFundService#listWithdraw(Integer, Integer, Long, Long)`）。
    async fn list_withdraw(
        &self,
        page_num: Option<i32>,
        page_size: Option<i32>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<WithdrawListResponse, WxErrorException>;

    /// 修改结算账户（对应 Java `WxChannelFundService#setBankAccount(AccountInfo)`）。
    async fn set_bank_account(
        &self,
        account_info: AccountInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 商户提现（对应 Java
    /// `WxChannelFundService#submitWithdraw(Integer, String, String)`）。
    ///
    /// # 参数
    /// - `amount`：提现金额（单位：分）
    /// - `remark`：提现备注
    /// - `bank_memo`：银行附言
    async fn submit_withdraw(
        &self,
        amount: Option<i32>,
        remark: String,
        bank_memo: String,
    ) -> Result<WithdrawSubmitResponse, WxErrorException>;

    /// 根据卡号查银行信息（对应 Java `WxChannelFundService#getBankInfoByCardNo(String)`）。
    ///
    /// # 参数
    /// - `account_number`：卡号
    async fn get_bank_info_by_card_no(
        &self,
        account_number: String,
    ) -> Result<BankInfoResponse, WxErrorException>;

    /// 搜索银行列表（对应 Java
    /// `WxChannelFundService#searchBankList(Integer, Integer, String, Integer)`）。
    async fn search_bank_list(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        keywords: String,
        bank_type: Option<i32>,
    ) -> Result<BankListResponse, WxErrorException>;

    /// 查询城市列表（对应 Java `WxChannelFundService#searchCityList(String)`）。
    ///
    /// # 参数
    /// - `province_code`：省份编码
    async fn search_city_list(
        &self,
        province_code: String,
    ) -> Result<BankCityResponse, WxErrorException>;

    /// 查询大陆银行省份列表（对应 Java `WxChannelFundService#getProvinceList()`）。
    async fn get_province_list(&self) -> Result<BankProvinceResponse, WxErrorException>;

    /// 查询支行列表（对应 Java
    /// `WxChannelFundService#searchBranchList(String, String, Integer, Integer)`）。
    async fn search_branch_list(
        &self,
        bank_code: String,
        city_code: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<BranchInfoResponse, WxErrorException>;

    /// 获取二维码（对应 Java `WxChannelFundService#getQrCode(String)`）。
    ///
    /// # 参数
    /// - `qrcode_ticket`：二维码 ticket
    async fn get_qr_code(&self, qrcode_ticket: String) -> Result<QrCodeResponse, WxErrorException>;

    /// 查询扫码状态（对应 Java `WxChannelFundService#checkQrStatus(String)`）。
    ///
    /// # 参数
    /// - `qrcode_ticket`：二维码 ticket
    async fn check_qr_status(
        &self,
        qrcode_ticket: String,
    ) -> Result<QrCheckResponse, WxErrorException>;
}
