//! 对应 Java `com.github.binarywang.wxpay.service.EcommerceService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;
// Java `EcommerceService` 使用 `bean.result.enums.TradeTypeEnum`（Rust 侧即 crate::enums::trade_type）
use crate::enums::trade_type::TradeTypeEnum;

/// EcommerceService（对应 Java `EcommerceService`）。
#[async_trait]
pub trait EcommerceService: Send + Sync {
    /// 电商收付通相关服务类. 产品介绍 created on 2020 /08/17
    async fn create_apply(
        &self,
        request: &ApplymentsRequest,
    ) -> Result<ApplymentsResult, WxErrorException>;

    /// 查询申请状态API 请求URL: https://api.mch.weixin.qq.com/v3/ecommerce/applyments/{applyment_id} 接口文档
    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplymentsStatusResult, WxErrorException>;

    /// 查询申请状态API 请求URL: https://api.mch.weixin.qq.com/v3/ecommerce/applyments/out-request-no/{out_request_n
    async fn query_apply_status_by_out_request_no(
        &self,
        out_request_no: &str,
    ) -> Result<ApplymentsStatusResult, WxErrorException>;

    /// 合单支付API(APP支付、JSAPI支付、H5支付、NATIVE支付). 请求URL：https://api.mch.weixin.qq.com/v3/combine-transactions/js
    async fn combine(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<CombineTransactionsResult, WxErrorException>;

    /// 合单支付API(APP支付、JSAPI支付、H5支付、NATIVE支付). 请求URL：https://api.mch.weixin.qq.com/v3/combine-transactions/js
    /// `ADAPTED`：Java 泛型 `<T> T` 返回值以 `serde_json::Value` 类型擦除。
    async fn combine_transactions(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<serde_json::Value, WxErrorException>;

    /// 合单支付通知回调数据处理 接口文档
    async fn parse_combine_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<CombineNotifyResult, WxErrorException>;

    /// 合单查询订单API 接口文档
    async fn query_combine(
        &self,
        combine_out_trade_no: &str,
    ) -> Result<CombineQueryResult, WxErrorException>;

    /// 合单关闭订单API 请求URL: https://api.mch.weixin.qq.com/v3/combine-transactions/out-trade-no/{combine_out_tra
    async fn close_combine(&self, request: &CombineCloseRequest) -> Result<(), WxErrorException>;

    /// 服务商模式普通支付API(APP支付、JSAPI支付、H5支付、NATIVE支付). 请求URL：https://api.mch.weixin.qq.com/v3/pay/partner/transa
    async fn unified_partner_order(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<WxPayUnifiedOrderV3Result, WxErrorException>;

    /// 服务商模式普通支付API(APP支付、JSAPI支付、H5支付、NATIVE支付). 请求URL：https://api.mch.weixin.qq.com/v3/pay/partner/transa
    /// `ADAPTED`：Java 泛型 `<T> T` 返回值以 `serde_json::Value` 类型擦除。
    async fn create_partner_order(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<serde_json::Value, WxErrorException>;

    /// 普通支付通知回调数据处理 接口文档
    async fn parse_partner_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayPartnerNotifyV3Result, WxErrorException>;

    /// 普通查询订单API 接口文档
    async fn query_partner_order(
        &self,
        request: &WxPayPartnerOrderQueryV3Request,
    ) -> Result<WxPayPartnerOrderQueryV3Result, WxErrorException>;

    /// 关闭普通订单API 接口文档
    async fn close_partner_order(
        &self,
        request: &WxPayPartnerOrderCloseV3Request,
    ) -> Result<(), WxErrorException>;

    /// 服务商账户实时余额 接口文档
    async fn sp_now_balance(
        &self,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException>;

    /// 服务商账户日终余额 接口文档
    async fn sp_day_end_balance(
        &self,
        account_type: SpAccountTypeEnum,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException>;

    /// 二级商户号账户实时余额 接口文档
    async fn sub_now_balance(&self, sub_mchid: &str)
    -> Result<FundBalanceResult, WxErrorException>;

    /// 二级商户号账户实时余额 接口文档
    async fn sub_now_balance_with_account_type(
        &self,
        sub_mchid: &str,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException>;

    /// 二级商户号账户日终余额 接口文档
    async fn sub_day_end_balance(
        &self,
        sub_mchid: &str,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException>;

    /// 请求分账API 接口文档
    async fn profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 查询分账结果API 接口文档
    async fn query_profit_sharing(
        &self,
        request: &ProfitSharingQueryRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 查询订单剩余待分金额API 接口文档
    async fn query_profit_sharing_orders_unsplit_amount(
        &self,
        request: &ProfitSharingOrdersUnSplitAmountRequest,
    ) -> Result<ProfitSharingOrdersUnSplitAmountResult, WxErrorException>;

    /// 添加分账接收方API 接口文档
    async fn add_receivers(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException>;

    /// 删除分账接收方API 接口文档
    async fn delete_receivers(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException>;

    /// 请求分账回退API 接口文档
    async fn return_orders(
        &self,
        request: &ReturnOrdersRequest,
    ) -> Result<ReturnOrdersResult, WxErrorException>;

    /// 查询分账回退API 接口文档
    async fn query_return_orders(
        &self,
        request: &ReturnOrdersQueryRequest,
    ) -> Result<ReturnOrdersResult, WxErrorException>;

    /// 完结分账API 接口文档
    async fn finish_order(
        &self,
        request: &FinishOrderRequest,
    ) -> Result<ProfitSharingResult, WxErrorException>;

    /// 退款申请API 接口文档
    async fn refunds(&self, request: &RefundsRequest) -> Result<RefundsResult, WxErrorException>;

    /// 查询退款API 接口文档
    async fn query_refund_by_refund_id(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<RefundQueryResult, WxErrorException>;

    /// 垫付退款回补API 接口文档
    async fn refunds_return_advance(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<ReturnAdvanceResult, WxErrorException>;

    /// 查询垫付回补结果API 接口文档
    async fn query_refunds_return_advance(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<ReturnAdvanceResult, WxErrorException>;

    /// 查询退款API 接口文档
    async fn query_refund_by_out_refund_no(
        &self,
        sub_mchid: &str,
        out_refund_no: &str,
    ) -> Result<RefundQueryResult, WxErrorException>;

    /// 退款通知回调数据处理 接口文档
    async fn parse_refund_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<RefundNotifyResult, WxErrorException>;

    /// 提现状态变更通知回调数据处理 接口文档
    async fn parse_withdraw_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WithdrawNotifyResult, WxErrorException>;

    /// 二级商户账户余额提现API 接口文档
    async fn sub_withdraw(
        &self,
        request: &SubWithdrawRequest,
    ) -> Result<SubWithdrawResult, WxErrorException>;

    /// 电商平台提现API 接口文档
    async fn sp_withdraw(
        &self,
        request: &SpWithdrawRequest,
    ) -> Result<SpWithdrawResult, WxErrorException>;

    /// 二级商户查询提现状态API 接口文档
    async fn query_sub_withdraw_by_out_request_no(
        &self,
        sub_mchid: &str,
        out_request_no: &str,
    ) -> Result<SubWithdrawStatusResult, WxErrorException>;

    /// 电商平台查询提现状态API 接口文档
    async fn query_sp_withdraw_by_out_request_no(
        &self,
        out_request_no: &str,
    ) -> Result<SpWithdrawStatusResult, WxErrorException>;

    /// 平台查询预约提现状态（根据微信支付预约提现单号查询） 接口文档
    async fn query_sp_withdraw_by_withdraw_id(
        &self,
        withdraw_id: &str,
    ) -> Result<SpWithdrawStatusResult, WxErrorException>;

    /// 二级商户按日终余额预约提现 接口文档
    async fn sub_day_end_balance_withdraw(
        &self,
        request: &SubDayEndBalanceWithdrawRequest,
    ) -> Result<SubDayEndBalanceWithdrawResult, WxErrorException>;

    /// 查询二级商户按日终余额预约提现状态 接口文档
    async fn query_sub_day_end_balance_withdraw(
        &self,
        sub_mchid: &str,
        out_request_no: &str,
    ) -> Result<SubDayEndBalanceWithdrawStatusResult, WxErrorException>;

    /// 修改结算账号API 接口文档
    async fn modify_settlement(
        &self,
        sub_mchid: &str,
        request: &SettlementRequest,
    ) -> Result<(), WxErrorException>;

    /// 查询结算账户API 接口文档
    async fn query_settlement(&self, sub_mchid: &str)
    -> Result<SettlementResult, WxErrorException>;

    /// 请求账单API 接口文档
    async fn apply_bill(
        &self,
        request: &TradeBillRequest,
    ) -> Result<TradeBillResult, WxErrorException>;

    /// 申请资金账单API 接口文档
    async fn apply_fund_bill(
        &self,
        bill_type: FundBillTypeEnum,
        request: &FundBillRequest,
    ) -> Result<FundBillResult, WxErrorException>;

    /// 下载账单API 接口文档
    /// `ADAPTED`：Java `InputStream` 返回值以 `Vec<u8>` 表达（下载字节）。
    async fn download_bill(&self, url: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 请求补差API 接口文档
    async fn subsidies_create(
        &self,
        subsidies_create_request: &SubsidiesCreateRequest,
    ) -> Result<SubsidiesCreateResult, WxErrorException>;

    /// 请求补差回退API 接口文档
    async fn subsidies_return(
        &self,
        subsidies_return_request: &SubsidiesReturnRequest,
    ) -> Result<SubsidiesReturnResult, WxErrorException>;

    /// 取消补差API 接口文档
    async fn subsidies_cancel(
        &self,
        subsidies_cancel_request: &SubsidiesCancelRequest,
    ) -> Result<SubsidiesCancelResult, WxErrorException>;

    /// 提交注销申请单 接口文档
    async fn created_account_cancel_application(
        &self,
        account_cancel_applications_request: &AccountCancelApplicationsRequest,
    ) -> Result<AccountCancelApplicationsResult, WxErrorException>;

    /// 查询注销单状态 接口文档
    async fn get_account_cancel_application(
        &self,
        out_apply_no: &str,
    ) -> Result<AccountCancelApplicationsResult, WxErrorException>;

    /// 注销单资料图片上传（对应 Java `uploadMediaAccountCancelApplication(File)`，
    /// 接口地址 `/v3/ecommerce/account/cancel-applications/media`，multipart）。
    ///
    /// `ADAPTED`：Java `File` 媒体参数以 `(文件名, 文件字节)` 表达。
    async fn upload_media_account_cancel_application(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<AccountCancelApplicationsMediaResult, WxErrorException>;
}
