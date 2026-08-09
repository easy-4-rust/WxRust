//! 小程序虚拟支付服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaXPayService`
//! （`impl.WxMaXPayServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxMaBaseResponse;
use crate::bean::xpay::{
    WxMaXPayBindTransferAccountRequest, WxMaXPayCancelCurrencyPayRequest,
    WxMaXPayCancelCurrencyPayResponse, WxMaXPayCompleteComplaintRequest,
    WxMaXPayCreateFundsBillRequest, WxMaXPayCreateFundsBillResponse,
    WxMaXPayCreateWithdrawOrderRequest, WxMaXPayCreateWithdrawOrderResponse,
    WxMaXPayCurrencyPayRequest, WxMaXPayCurrencyPayResponse,
    WxMaXPayDownloadAdverfundsOrderRequest, WxMaXPayDownloadAdverfundsOrderResponse,
    WxMaXPayDownloadBillRequest, WxMaXPayDownloadBillResponse, WxMaXPayGetComplaintDetailRequest,
    WxMaXPayGetComplaintDetailResponse, WxMaXPayGetComplaintListRequest,
    WxMaXPayGetComplaintListResponse, WxMaXPayGetNegotiationHistoryRequest,
    WxMaXPayGetNegotiationHistoryResponse, WxMaXPayGetUploadFileSignRequest,
    WxMaXPayGetUploadFileSignResponse, WxMaXPayNotifyProvideGoodsRequest,
    WxMaXPayPresentCurrencyRequest, WxMaXPayPresentCurrencyResponse, WxMaXPayPresentGoodsRequest,
    WxMaXPayPresentGoodsResponse, WxMaXPayQueryAdverFundsRequest, WxMaXPayQueryAdverFundsResponse,
    WxMaXPayQueryBizBalanceRequest, WxMaXPayQueryBizBalanceResponse, WxMaXPayQueryFundsBillRequest,
    WxMaXPayQueryFundsBillResponse, WxMaXPayQueryOrderRequest, WxMaXPayQueryOrderResponse,
    WxMaXPayQueryPublishGoodsRequest, WxMaXPayQueryPublishGoodsResponse,
    WxMaXPayQueryRecoverBillRequest, WxMaXPayQueryRecoverBillResponse,
    WxMaXPayQueryTransferAccountRequest, WxMaXPayQueryTransferAccountResponse,
    WxMaXPayQueryUploadGoodsRequest, WxMaXPayQueryUploadGoodsResponse,
    WxMaXPayQueryUserBalanceRequest, WxMaXPayQueryUserBalanceResponse,
    WxMaXPayQueryWithdrawOrderRequest, WxMaXPayQueryWithdrawOrderResponse,
    WxMaXPayRefundOrderRequest, WxMaXPayRefundOrderResponse, WxMaXPayResponseComplaintRequest,
    WxMaXPaySigParams, WxMaXPayStartPublishGoodsRequest, WxMaXPayStartUploadGoodsRequest,
    WxMaXPayUploadVpFileRequest, WxMaXPayUploadVpFileResponse,
};

/// 小程序虚拟支付服务。
///
/// 对应 Java `WxMaXPayService`：虚拟币余额/充值/赠送、订单查询/取消/退款、
/// 发货通知、道具直购/上传/发布、提现、对账单、广告金、投诉与媒体上传。
/// 所有方法均以请求体做 HMAC-SHA256 签名后拼接 `pay_sig`/`signature` 到
/// URL（对应 Java `WxMaXPaySigParams.signUriWithPay/signUriWithBoth`）。
#[async_trait]
pub trait WxMaXPayService: Send + Sync {
    /// 查询用户虚拟币余额（对应 Java `queryUserBalance`，双签名）。
    async fn query_user_balance(
        &self,
        request: &WxMaXPayQueryUserBalanceRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryUserBalanceResponse, WxErrorException>;

    /// 虚拟币充值下单（对应 Java `currencyPay`，双签名）。
    async fn currency_pay(
        &self,
        request: &WxMaXPayCurrencyPayRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCurrencyPayResponse, WxErrorException>;

    /// 查询订单信息（对应 Java `queryOrder`）。
    async fn query_order(
        &self,
        request: &WxMaXPayQueryOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryOrderResponse, WxErrorException>;

    /// 取消虚拟币充值订单（对应 Java `cancelCurrencyPay`，双签名）。
    async fn cancel_currency_pay(
        &self,
        request: &WxMaXPayCancelCurrencyPayRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCancelCurrencyPayResponse, WxErrorException>;

    /// 通知发货（对应 Java `notifyProvideGoods`）。
    async fn notify_provide_goods(
        &self,
        request: &WxMaXPayNotifyProvideGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException>;

    /// 赠送虚拟币（对应 Java `presentCurrency`）。
    async fn present_currency(
        &self,
        request: &WxMaXPayPresentCurrencyRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayPresentCurrencyResponse, WxErrorException>;

    /// 道具直购（对应 Java `presentGoods`）。
    async fn present_goods(
        &self,
        request: &WxMaXPayPresentGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayPresentGoodsResponse, WxErrorException>;

    /// 下载对账单（对应 Java `downloadBill`）。
    async fn download_bill(
        &self,
        request: &WxMaXPayDownloadBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayDownloadBillResponse, WxErrorException>;

    /// 退款申请（对应 Java `refundOrder`）。
    async fn refund_order(
        &self,
        request: &WxMaXPayRefundOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayRefundOrderResponse, WxErrorException>;

    /// 创建提现订单（对应 Java `createWithdrawOrder`）。
    async fn create_withdraw_order(
        &self,
        request: &WxMaXPayCreateWithdrawOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCreateWithdrawOrderResponse, WxErrorException>;

    /// 查询提现订单（对应 Java `queryWithdrawOrder`）。
    async fn query_withdraw_order(
        &self,
        request: &WxMaXPayQueryWithdrawOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryWithdrawOrderResponse, WxErrorException>;

    /// 启动道具上传（对应 Java `startUploadGoods`）。
    async fn start_upload_goods(
        &self,
        request: &WxMaXPayStartUploadGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException>;

    /// 查询道具上传状态（对应 Java `queryUploadGoods`）。
    async fn query_upload_goods(
        &self,
        request: &WxMaXPayQueryUploadGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryUploadGoodsResponse, WxErrorException>;

    /// 启动道具发布（对应 Java `startPublishGoods`）。
    async fn start_publish_goods(
        &self,
        request: &WxMaXPayStartPublishGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<bool, WxErrorException>;

    /// 查询道具发布状态（对应 Java `queryPublishGoods`）。
    async fn query_publish_goods(
        &self,
        request: &WxMaXPayQueryPublishGoodsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryPublishGoodsResponse, WxErrorException>;

    /// 查询商家账户可提现余额（对应 Java `queryBizBalance`）。
    async fn query_biz_balance(
        &self,
        request: &WxMaXPayQueryBizBalanceRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryBizBalanceResponse, WxErrorException>;

    /// 查询广告金充值账户（对应 Java `queryTransferAccount`）。
    async fn query_transfer_account(
        &self,
        request: &WxMaXPayQueryTransferAccountRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryTransferAccountResponse, WxErrorException>;

    /// 查询广告金发放记录（对应 Java `queryAdverFunds`）。
    async fn query_adver_funds(
        &self,
        request: &WxMaXPayQueryAdverFundsRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryAdverFundsResponse, WxErrorException>;

    /// 充值广告金（对应 Java `createFundsBill`）。
    async fn create_funds_bill(
        &self,
        request: &WxMaXPayCreateFundsBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayCreateFundsBillResponse, WxErrorException>;

    /// 绑定广告金充值账户（对应 Java `bindTransferAccount`）。
    async fn bind_transfer_account(
        &self,
        request: &WxMaXPayBindTransferAccountRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException>;

    /// 查询广告金充值记录（对应 Java `queryFundsBill`）。
    async fn query_funds_bill(
        &self,
        request: &WxMaXPayQueryFundsBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryFundsBillResponse, WxErrorException>;

    /// 查询广告金回收记录（对应 Java `queryRecoverBill`）。
    async fn query_recover_bill(
        &self,
        request: &WxMaXPayQueryRecoverBillRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayQueryRecoverBillResponse, WxErrorException>;

    /// 获取投诉列表（对应 Java `getComplaintList`）。
    async fn get_complaint_list(
        &self,
        request: &WxMaXPayGetComplaintListRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetComplaintListResponse, WxErrorException>;

    /// 获取投诉详情（对应 Java `getComplaintDetail`）。
    async fn get_complaint_detail(
        &self,
        request: &WxMaXPayGetComplaintDetailRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetComplaintDetailResponse, WxErrorException>;

    /// 获取协商历史（对应 Java `getNegotiationHistory`）。
    async fn get_negotiation_history(
        &self,
        request: &WxMaXPayGetNegotiationHistoryRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetNegotiationHistoryResponse, WxErrorException>;

    /// 回复用户（对应 Java `responseComplaint`）。
    async fn response_complaint(
        &self,
        request: &WxMaXPayResponseComplaintRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException>;

    /// 完成投诉处理（对应 Java `completeComplaint`）。
    async fn complete_complaint(
        &self,
        request: &WxMaXPayCompleteComplaintRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaBaseResponse, WxErrorException>;

    /// 上传媒体文件（对应 Java `uploadVpFile`）。
    async fn upload_vp_file(
        &self,
        request: &WxMaXPayUploadVpFileRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayUploadVpFileResponse, WxErrorException>;

    /// 获取微信支付反馈投诉图片的签名头部（对应 Java `getUploadFileSign`）。
    async fn get_upload_file_sign(
        &self,
        request: &WxMaXPayGetUploadFileSignRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayGetUploadFileSignResponse, WxErrorException>;

    /// 下载广告金对应的商户订单信息（对应 Java `downloadAdverfundsOrder`）。
    async fn download_adverfunds_order(
        &self,
        request: &WxMaXPayDownloadAdverfundsOrderRequest,
        sig_params: &WxMaXPaySigParams,
    ) -> Result<WxMaXPayDownloadAdverfundsOrderResponse, WxErrorException>;
}
