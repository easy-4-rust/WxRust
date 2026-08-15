//! 对应 Java `service.impl.EcommerceServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::EcommerceService;
use crate::api::WxPayService;
use crate::bean::*;
use crate::enums::trade_type::TradeTypeEnum;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// 电商收付通服务实现（对应 Java `EcommerceServiceImpl`）。
pub struct EcommerceServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl EcommerceServiceImpl {
    /// 构建实现（对应 Java 构造器 `EcommerceServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }
}

#[async_trait]
impl EcommerceService for EcommerceServiceImpl {
    async fn create_apply(
        &self,
        request: &ApplymentsRequest,
    ) -> Result<ApplymentsResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`（@SpecEncrypt 字段）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &[
                "id_card_info.id_card_name",
                "id_card_info.id_card_number",
                "id_card_info.id_card_address",
                "id_doc_info.id_doc_name",
                "id_doc_info.id_doc_number",
                "id_doc_info.id_doc_address",
                "ubo_info_list.*.ubo_id_doc_name",
                "ubo_info_list.*.ubo_id_doc_number",
                "ubo_info_list.*.ubo_id_doc_address",
                "account_info.account_name",
                "account_info.account_number",
                "contact_info.contact_name",
                "contact_info.contact_id_card_number",
                "contact_info.mobile_phone",
                "contact_info.contact_email",
            ],
        )?;
        let url = format!("{}/v3/ecommerce/applyments/", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplymentsStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/applyments/{applyment_id}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_apply_status_by_out_request_no(
        &self,
        out_request_no: &str,
    ) -> Result<ApplymentsStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/applyments/out-request-no/{out_request_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn combine(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<CombineTransactionsResult, WxErrorException> {
        // 对应 Java：委托门面 `combine(tradeType, request)`
        let svc = self.svc()?;
        let _ = trade_type;
        svc.combine(trade_type, request).await
    }

    async fn combine_transactions(
        &self,
        trade_type: TradeTypeEnum,
        request: &CombineTransactionsRequest,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self.svc()?;
        svc.combine_transactions(trade_type, request).await
    }

    async fn parse_combine_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<CombineNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        svc.parse_combine_notify_result(notify_data, header).await
    }

    async fn query_combine(
        &self,
        combine_out_trade_no: &str,
    ) -> Result<CombineQueryResult, WxErrorException> {
        let svc = self.svc()?;
        svc.query_combine(combine_out_trade_no).await
    }

    async fn close_combine(&self, request: &CombineCloseRequest) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        svc.close_combine(request).await
    }

    async fn unified_partner_order(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<WxPayUnifiedOrderV3Result, WxErrorException> {
        let svc = self.svc()?;
        svc.unified_partner_order_v3(trade_type, request).await
    }

    async fn create_partner_order(
        &self,
        trade_type: TradeTypeEnum,
        request: &WxPayPartnerUnifiedOrderV3Request,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self.svc()?;
        svc.create_partner_order_v3(trade_type, request).await
    }

    async fn parse_partner_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WxPayPartnerNotifyV3Result, WxErrorException> {
        let svc = self.svc()?;
        svc.parse_partner_order_notify_v3_result(notify_data, header)
            .await
    }

    async fn query_partner_order(
        &self,
        request: &WxPayPartnerOrderQueryV3Request,
    ) -> Result<WxPayPartnerOrderQueryV3Result, WxErrorException> {
        // 对应 Java 门面 `queryPartnerOrderV3(request)`；Rust 门面签名以
        // transaction_id/out_trade_no 二选一表达（ADAPTED）
        let svc = self.svc()?;
        svc.query_partner_order_v3(
            request.transaction_id.as_deref(),
            request.out_trade_no.as_deref(),
        )
        .await
    }

    async fn close_partner_order(
        &self,
        request: &WxPayPartnerOrderCloseV3Request,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        svc.close_partner_order_v3(request.out_trade_no.as_deref().unwrap_or_default())
            .await
    }

    async fn sp_now_balance(
        &self,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant/fund/balance/{}",
            svc.get_pay_base_url(),
            account_type.value()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sp_day_end_balance(
        &self,
        account_type: SpAccountTypeEnum,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/merchant/fund/dayendbalance/{}?date={date}",
            svc.get_pay_base_url(),
            account_type.value()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sub_now_balance(
        &self,
        sub_mchid: &str,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/balance/{sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sub_now_balance_with_account_type(
        &self,
        sub_mchid: &str,
        account_type: SpAccountTypeEnum,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/ecommerce/fund/balance/{sub_mchid}",
            svc.get_pay_base_url()
        );
        url.push_str(&format!("?account_type={}", account_type.value()));
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sub_day_end_balance(
        &self,
        sub_mchid: &str,
        date: &str,
    ) -> Result<FundBalanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/enddaybalance/{sub_mchid}?date={date}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`（receivers[*].receiver_name）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(
            &mut json,
            &public_key,
            &["receivers.*.receiver_name"],
        )?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/orders",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_profit_sharing(
        &self,
        request: &ProfitSharingQueryRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/orders?sub_mchid={}&transaction_id={}&out_order_no={}",
            svc.get_pay_base_url(),
            request.sub_mchid.as_deref().unwrap_or_default(),
            request.transaction_id.as_deref().unwrap_or_default(),
            request.out_order_no.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_profit_sharing_orders_unsplit_amount(
        &self,
        request: &ProfitSharingOrdersUnSplitAmountRequest,
    ) -> Result<ProfitSharingOrdersUnSplitAmountResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/orders/{}/amounts",
            svc.get_pay_base_url(),
            request.transaction_id.as_deref().unwrap_or_default()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn add_receivers(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/receivers/add",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn delete_receivers(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/receivers/delete",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn return_orders(
        &self,
        request: &ReturnOrdersRequest,
    ) -> Result<ReturnOrdersResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/returnorders",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_return_orders(
        &self,
        request: &ReturnOrdersQueryRequest,
    ) -> Result<ReturnOrdersResult, WxErrorException> {
        let svc = self.svc()?;
        let sub_mchid = request.sub_mchid.as_deref().unwrap_or_default();
        let out_return_no = request.out_return_no.as_deref().unwrap_or_default();
        let url = if request
            .order_id
            .as_deref()
            .map(str::trim)
            .unwrap_or_default()
            .is_empty()
        {
            format!(
                "{}/v3/ecommerce/profitsharing/returnorders?sub_mchid={sub_mchid}&out_order_no={}&out_return_no={out_return_no}",
                svc.get_pay_base_url(),
                request.out_order_no.as_deref().unwrap_or_default()
            )
        } else {
            format!(
                "{}/v3/ecommerce/profitsharing/returnorders?sub_mchid={sub_mchid}&order_id={}&out_return_no={out_return_no}",
                svc.get_pay_base_url(),
                request.order_id.as_deref().unwrap_or_default()
            )
        };
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn finish_order(
        &self,
        request: &FinishOrderRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/profitsharing/finish-order",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn refunds(&self, request: &RefundsRequest) -> Result<RefundsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/ecommerce/refunds/apply", svc.get_pay_base_url());
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_refund_by_refund_id(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<RefundQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/refunds/id/{refund_id}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn refunds_return_advance(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<ReturnAdvanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/refunds/{refund_id}/return-advance",
            svc.get_pay_base_url()
        );
        let body = serde_json::json!({ "sub_mchid": sub_mchid });
        let response = svc.post_v3(&url, &body.to_string()).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_refunds_return_advance(
        &self,
        sub_mchid: &str,
        refund_id: &str,
    ) -> Result<ReturnAdvanceResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/refunds/{refund_id}/return-advance?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_refund_by_out_refund_no(
        &self,
        sub_mchid: &str,
        out_refund_no: &str,
    ) -> Result<RefundQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/refunds/out-refund-no/{out_refund_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_refund_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<RefundNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn parse_withdraw_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<WithdrawNotifyResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            api_v3_key,
            move |_serial, message, signature| {
                crate::util::crypto::wx_pay_v3_crypto_utils::verify_sha256_rsa(
                    &public_key,
                    message,
                    signature,
                )
                .unwrap_or(false)
            },
        )
        .map_err(|e| impl_utils::runtime(format!("解析报文异常！: {e}")))?;
        Ok(parsed.result)
    }

    async fn sub_withdraw(
        &self,
        request: &SubWithdrawRequest,
    ) -> Result<SubWithdrawResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/submch/withdraw",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sp_withdraw(
        &self,
        request: &SpWithdrawRequest,
    ) -> Result<SpWithdrawResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/ecommerce/fund/withdraw", svc.get_pay_base_url());
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_sub_withdraw_by_out_request_no(
        &self,
        sub_mchid: &str,
        out_request_no: &str,
    ) -> Result<SubWithdrawStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/submch/withdraw/out-request-no/{out_request_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_sp_withdraw_by_out_request_no(
        &self,
        out_request_no: &str,
    ) -> Result<SpWithdrawStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/withdraw/out-request-no/{out_request_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_sp_withdraw_by_withdraw_id(
        &self,
        withdraw_id: &str,
    ) -> Result<SpWithdrawStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/withdraw/withdraw-id/{withdraw_id}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn sub_day_end_balance_withdraw(
        &self,
        request: &SubDayEndBalanceWithdrawRequest,
    ) -> Result<SubDayEndBalanceWithdrawResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/submch/withdraw/enddaybalance",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn query_sub_day_end_balance_withdraw(
        &self,
        sub_mchid: &str,
        out_request_no: &str,
    ) -> Result<SubDayEndBalanceWithdrawStatusResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/fund/submch/withdraw/enddaybalance/out-request-no/{out_request_no}?sub_mchid={sub_mchid}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn modify_settlement(
        &self,
        sub_mchid: &str,
        request: &SettlementRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`（account_number）
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["account_number"])?;
        let url = format!(
            "{}/v3/ecommerce/applyments/{sub_mchid}/settlement",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        Ok(())
    }

    async fn query_settlement(
        &self,
        sub_mchid: &str,
    ) -> Result<SettlementResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/applyments/{sub_mchid}/settlement",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn apply_bill(
        &self,
        request: &TradeBillRequest,
    ) -> Result<TradeBillResult, WxErrorException> {
        let svc = self.svc()?;
        // 对应 Java `parseURLPair(request)`：非空字段 LOWER_UNDERSCORE 查询串
        let query = query_from_request(request)?;
        let url = format!("{}/v3/bill/tradebill?{query}", svc.get_pay_base_url());
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn apply_fund_bill(
        &self,
        bill_type: FundBillTypeEnum,
        request: &FundBillRequest,
    ) -> Result<FundBillResult, WxErrorException> {
        let svc = self.svc()?;
        let query = query_from_request(request)?;
        let template = bill_type.url();
        let url = template
            .replacen("%s", &svc.get_pay_base_url(), 1)
            .replacen("%s", &query, 1);
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn download_bill(&self, url: &str) -> Result<Vec<u8>, WxErrorException> {
        // 对应 Java `payService.downloadV3(url)`（账单文件下载）
        let svc = self.svc()?;
        svc.download_v3(url).await
    }

    async fn subsidies_create(
        &self,
        subsidies_create_request: &SubsidiesCreateRequest,
    ) -> Result<SubsidiesCreateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/ecommerce/subsidies/create", svc.get_pay_base_url());
        let body = serde_json::to_string(subsidies_create_request)
            .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn subsidies_return(
        &self,
        subsidies_return_request: &SubsidiesReturnRequest,
    ) -> Result<SubsidiesReturnResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/ecommerce/subsidies/return", svc.get_pay_base_url());
        let body = serde_json::to_string(subsidies_return_request)
            .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn subsidies_cancel(
        &self,
        subsidies_cancel_request: &SubsidiesCancelRequest,
    ) -> Result<SubsidiesCancelResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}/v3/ecommerce/subsidies/cancel", svc.get_pay_base_url());
        let body = serde_json::to_string(subsidies_cancel_request)
            .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn created_account_cancel_application(
        &self,
        account_cancel_applications_request: &AccountCancelApplicationsRequest,
    ) -> Result<AccountCancelApplicationsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/account/cancel-applications",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(account_cancel_applications_request)
            .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_account_cancel_application(
        &self,
        out_apply_no: &str,
    ) -> Result<AccountCancelApplicationsResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/account/cancel-applications/out-apply-no/{out_apply_no}",
            svc.get_pay_base_url()
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn upload_media_account_cancel_application(
        &self,
        file_name: &str,
        file_data: &[u8],
    ) -> Result<AccountCancelApplicationsMediaResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/ecommerce/account/cancel-applications/media",
            svc.get_pay_base_url()
        );
        // 对应 Java `WechatPayUploadHttpPost.Builder.withImage(...).buildEcommerceAccount()`：
        // meta 使用 file_name/file_digest 键名
        let sha256 = crate::util::crypto::wx_pay_crypto_utils::sha256_hex(file_data);
        let meta = format!("{{\"file_name\":\"{file_name}\",\"file_digest\":\"{sha256}\"}}");
        let (content_type, body) = impl_utils::build_multipart_meta_file(
            file_name,
            impl_utils::guess_file_content_type(file_name),
            file_data,
            &meta,
        );
        let result = impl_utils::execute_v3_upload(
            svc.wx_pay_config().as_ref(),
            svc.http_client(),
            &url,
            &content_type,
            &body,
        )
        .await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

/// 请求 bean → 查询串（对应 Java `parseURLPair`：非空字段、
/// LOWER_CAMEL→LOWER_UNDERSCORE，`class` 跳过）。
///
/// `ADAPTED`：以 serde 序列化（字段名即 `@SerializedName` 下划线形式）取
/// 非 null 字段拼接 `k=v&...`。
fn query_from_request<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
    let value = serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
    let Some(obj) = value.as_object() else {
        return Ok(String::new());
    };
    let mut parts = Vec::new();
    for (k, v) in obj {
        if k == "class" {
            continue;
        }
        if let Some(s) = v.as_str() {
            if !s.is_empty() {
                parts.push(format!("{k}={s}"));
            }
        } else if !v.is_null() {
            parts.push(format!("{k}={v}"));
        }
    }
    Ok(parts.join("&"))
}
