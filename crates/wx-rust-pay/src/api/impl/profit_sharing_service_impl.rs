//! 对应 Java `service.impl.ProfitSharingServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{ProfitSharingService, WxPayService};
use crate::bean::*;
// 注意：`crate::bean` 扁平重导出的 ProfitSharingRequest/ProfitSharingResult 等为
// ecommerce 包同名类型（生成器冲突取舍），此处以全限定路径显式导入 profitsharing 包版本。
use crate::bean::profitsharing::request::profit_sharing_merchant_ratio_query_request::ProfitSharingMerchantRatioQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_order_amount_query_request::ProfitSharingOrderAmountQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_query_request::ProfitSharingQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_receiver_request::ProfitSharingReceiverRequest;
use crate::bean::profitsharing::request::profit_sharing_request::ProfitSharingRequest;
use crate::bean::profitsharing::request::profit_sharing_return_query_request::ProfitSharingReturnQueryRequest;
use crate::bean::profitsharing::request::profit_sharing_return_request::ProfitSharingReturnRequest;
use crate::bean::profitsharing::result::profit_sharing_receiver_result::ProfitSharingReceiverResult;
use crate::bean::profitsharing::result::profit_sharing_result::ProfitSharingResult;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// ProfitSharingService 实现（对应 Java `ProfitSharingServiceImpl`）。
pub struct ProfitSharingServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl ProfitSharingServiceImpl {
    /// 构建实现（对应 Java 构造器 `ProfitSharingServiceImpl(WxPayService)`）。
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
impl ProfitSharingService for ProfitSharingServiceImpl {
    async fn profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        self.post_v2_secapi(request, "/secapi/pay/profitsharing")
            .await
    }

    async fn multi_profit_sharing(
        &self,
        request: &ProfitSharingRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        self.post_v2_secapi(request, "/secapi/pay/multiprofitsharing")
            .await
    }

    async fn profit_sharing_v3(
        &self,
        request: &ProfitSharingV3Request,
    ) -> Result<ProfitSharingV3Result, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        // 对应 Java `RsaCryptoUtil.encryptFields`：receivers[*].name
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["receivers.*.name"])?;
        let url = format!("{}/v3/profitsharing/orders", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_finish(
        &self,
        request: &ProfitSharingUnfreezeRequest,
    ) -> Result<ProfitSharingResult, WxErrorException> {
        self.post_v2_secapi(request, "/secapi/pay/profitsharingfinish")
            .await
    }

    async fn add_receiver(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException> {
        self.post_v2_secapi(request, "/pay/profitsharingaddreceiver")
            .await
    }

    async fn remove_receiver(
        &self,
        request: &ProfitSharingReceiverRequest,
    ) -> Result<ProfitSharingReceiverResult, WxErrorException> {
        self.post_v2_secapi(request, "/pay/profitsharingremovereceiver")
            .await
    }

    async fn add_receiver_v3(
        &self,
        request: &ProfitSharingReceiverV3Request,
    ) -> Result<ProfitSharingReceiverV3Result, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["name"])?;
        let url = format!("{}/v3/profitsharing/receivers/add", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn remove_receiver_v3(
        &self,
        request: &ProfitSharingReceiverV3Request,
    ) -> Result<ProfitSharingReceiverV3Result, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["name"])?;
        let url = format!(
            "{}/v3/profitsharing/receivers/delete",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_query(
        &self,
        request: &ProfitSharingQueryRequest,
    ) -> Result<ProfitSharingQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        // 对应 Java：appid 置空后签名（不携带 appid）
        request.appid = None;
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/profitsharingquery", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        let mut result: ProfitSharingQueryResult = impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            ProfitSharingQueryResult::from_xml,
        )?;
        // 对应 Java `result.formatReceivers()`：receivers_json（LOWER_CASE_WITH_UNDERSCORES
        // JSON）解析为接收人列表
        if let Some(json) = result.receivers_json.clone() {
            if !json.trim().is_empty() {
                if let Ok(list) = serde_json::from_str::<
                    Vec<crate::bean::profitsharing::result::profit_sharing_query_result::Receiver>,
                >(&json)
                {
                    result.receivers = list;
                }
            }
        }
        Ok(result)
    }

    async fn profit_sharing_query_v3(
        &self,
        out_order_no: &str,
        transaction_id: &str,
    ) -> Result<ProfitSharingV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/orders/{out_order_no}?transaction_id={transaction_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_query_v3_with_account_type(
        &self,
        out_order_no: &str,
        transaction_id: &str,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/orders/{out_order_no}?sub_mchid={sub_mch_id}&transaction_id={transaction_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_query_v3_with_request(
        &self,
        request: &ProfitSharingQueryV3Request,
    ) -> Result<ProfitSharingV3Result, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/profitsharing/orders/{}?transaction_id={}",
            svc.get_pay_base_url(),
            request.out_order_no.as_deref().unwrap_or_default(),
            request.transaction_id.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.sub_mch_id.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("&sub_mchid={v}"));
            }
        }
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_order_amount_query(
        &self,
        request: &ProfitSharingOrderAmountQueryRequest,
    ) -> Result<ProfitSharingOrderAmountQueryResult, WxErrorException> {
        self.post_v2_secapi(request, "/pay/profitsharingorderamountquery")
            .await
    }

    async fn profit_sharing_unsplit_amount_query_v3(
        &self,
        transaction_id: &str,
    ) -> Result<ProfitSharingOrderAmountQueryV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/transactions/{transaction_id}/amounts",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_merchant_ratio_query(
        &self,
        request: &ProfitSharingMerchantRatioQueryRequest,
    ) -> Result<ProfitSharingMerchantRatioQueryResult, WxErrorException> {
        self.post_v2_secapi(request, "/pay/profitsharingmerchantratioquery")
            .await
    }

    async fn profit_sharing_merchant_ratio_query_v3(
        &self,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingMerchantRatioQueryV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/merchant-configs/{sub_mch_id}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_return(
        &self,
        return_request: &ProfitSharingReturnRequest,
    ) -> Result<ProfitSharingReturnResult, WxErrorException> {
        self.post_v2_secapi(return_request, "/secapi/pay/profitsharingreturn")
            .await
    }

    async fn profit_sharing_return_v3(
        &self,
        request: &ProfitSharingReturnV3Request,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["name"])?;
        let url = format!("{}/v3/profitsharing/return-orders", svc.get_pay_base_url());
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_return_query(
        &self,
        query_request: &ProfitSharingReturnQueryRequest,
    ) -> Result<ProfitSharingReturnResult, WxErrorException> {
        self.post_v2_secapi(query_request, "/pay/profitsharingreturnquery")
            .await
    }

    async fn profit_sharing_return_query_v3(
        &self,
        out_order_no: &str,
        out_return_no: &str,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/return-orders/{out_return_no}?out_order_no={out_order_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_return_query_v3_with_account_type(
        &self,
        out_order_no: &str,
        out_return_no: &str,
        sub_mch_id: &str,
    ) -> Result<ProfitSharingReturnV3Result, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/profitsharing/return-orders/{out_return_no}?sub_mchid={sub_mch_id}&out_order_no={out_order_no}",
            svc.get_pay_base_url()
        );
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn profit_sharing_unfreeze(
        &self,
        request: &ProfitSharingUnfreezeV3Request,
    ) -> Result<ProfitSharingUnfreezeV3Result, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut json =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        impl_utils::encrypt_spec_fields_json(&mut json, &public_key, &["name"])?;
        let url = format!(
            "{}/v3/profitsharing/orders/unfreeze",
            svc.get_pay_base_url()
        );
        let body = serde_json::to_string(&json).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let result = svc.post_v3_with_wechatpay_serial(&url, &body).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn parse_profit_sharing_notify_result(
        &self,
        notify_data: &str,
        header: &SignatureHeader,
    ) -> Result<ProfitSharingNotifyV3Result, WxErrorException> {
        // 对应 Java：验签（探测流量识别）→ 解析 → AES-GCM 解密 → 反序列化
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let public_key = impl_utils::platform_public_key(config.as_ref())?;
        let api_v3_key = config.api_v3_key().unwrap_or_default();
        let parsed = crate::util::wx_pay_notify_utils::parse_notify_v3_result(
            notify_data,
            Some(header),
            &api_v3_key,
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

    async fn profit_sharing_bill(
        &self,
        request: &ProfitSharingBillV3Request,
    ) -> Result<ProfitSharingBillV3Result, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}/v3/profitsharing/bills?bill_date={}",
            svc.get_pay_base_url(),
            request.bill_date.as_deref().unwrap_or_default()
        );
        if let Some(v) = request.sub_mch_id.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("&sub_mchid={v}"));
            }
        }
        if let Some(v) = request.tar_type.as_deref() {
            if !v.trim().is_empty() {
                url.push_str(&format!("&tar_type={v}"));
            }
        }
        let result = svc.get_v3(&url).await?;
        serde_json::from_str(&result).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}

impl ProfitSharingServiceImpl {
    /// v2 分账请求（对应 Java `request.checkAndSign` + `post(url, toXML, true)` +
    /// `BaseWxPayResult.fromXML` + `checkResult`）。
    async fn post_v2_secapi<T, R>(
        &self,
        request: &T,
        url_suffix: &str,
    ) -> Result<R, WxErrorException>
    where
        T: V2Request + Clone,
        R: FromXml,
    {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}{url_suffix}", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, true)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type().as_deref(),
            true,
            R::from_xml,
        )
    }
}

/// v2 XML 结果解析特征（对应 Java `BaseWxPayResult.fromXML`；本文件内为
/// 各结果类型 from_xml 的局部统一入口）。
trait FromXml: Sized {
    fn from_xml(xml: &str) -> Result<Self, String>;
}

impl FromXml for ProfitSharingResult {
    fn from_xml(xml: &str) -> Result<Self, String> {
        ProfitSharingResult::from_xml(xml)
    }
}
impl FromXml for ProfitSharingReceiverResult {
    fn from_xml(xml: &str) -> Result<Self, String> {
        ProfitSharingReceiverResult::from_xml(xml)
    }
}
impl FromXml for ProfitSharingOrderAmountQueryResult {
    fn from_xml(xml: &str) -> Result<Self, String> {
        ProfitSharingOrderAmountQueryResult::from_xml(xml)
    }
}
impl FromXml for ProfitSharingMerchantRatioQueryResult {
    fn from_xml(xml: &str) -> Result<Self, String> {
        ProfitSharingMerchantRatioQueryResult::from_xml(xml)
    }
}
impl FromXml for ProfitSharingReturnResult {
    fn from_xml(xml: &str) -> Result<Self, String> {
        ProfitSharingReturnResult::from_xml(xml)
    }
}
