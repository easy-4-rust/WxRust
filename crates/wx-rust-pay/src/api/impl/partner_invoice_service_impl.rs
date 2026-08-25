//! 服务商电子发票服务实现。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.impl.PartnerInvoiceServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use serde_json::json;
use wx_rust_common::error::WxErrorException;

use crate::api::{PartnerInvoiceService, WxPayService};
use crate::bean::invoice::buyer_information::BuyerInformation;
use crate::bean::invoice::card_template_request::CardTemplateRequest;
use crate::bean::invoice::card_template_result::CardTemplateResult;
use crate::bean::invoice::development_config_request::DevelopmentConfigRequest;
use crate::bean::invoice::development_config_result::DevelopmentConfigResult;
use crate::bean::invoice::general_invoice_request::GeneralInvoiceRequest;
use crate::bean::invoice::industry_invoice_request::IndustryInvoiceRequest;
use crate::bean::invoice::insert_card_request::InsertCardRequest;
use crate::bean::invoice::invite_merchant_query::InviteMerchantQuery;
use crate::bean::invoice::invite_merchant_result::InviteMerchantResult;
use crate::bean::invoice::invite_url_request::InviteUrlRequest;
use crate::bean::invoice::invite_url_result::InviteUrlResult;
use crate::bean::invoice::invoice_file_result::InvoiceFileResult;
use crate::bean::invoice::invoice_file_upload_request::InvoiceFileUploadRequest;
use crate::bean::invoice::invoice_file_upload_result::InvoiceFileUploadResult;
use crate::bean::invoice::invoice_result::InvoiceResult;
use crate::bean::invoice::reverse_invoice_request::ReverseInvoiceRequest;
use crate::bean::invoice::sub_merchant_invoice_status::SubMerchantInvoiceStatus;
use crate::bean::invoice::title_url_request::TitleUrlRequest;
use crate::bean::invoice::title_url_result::TitleUrlResult;
use crate::util::wx_pay_service_impl_utils as impl_utils;

/// URL 编码（对应 Java `URLEncoder.encode(s, "UTF-8")`）。
fn urlencoding(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// URL 常量（对应 Java `PartnerInvoiceServiceImpl` 的路径常量）。
const INVITE_URL_PATH: &str = "/v3/new-tax-control-fapiao/fapiaomerchant/getspinviteurl";
const ISSUE_GENERAL_PATH: &str = "/v3/new-tax-control-fapiao/fapiao-applications/issue-general";
const FAPIAO_APPLICATIONS_PATH: &str = "/v3/new-tax-control-fapiao/fapiao-applications/";

/// PartnerInvoiceService 实现（对应 Java `PartnerInvoiceServiceImpl`）。
pub struct PartnerInvoiceServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl PartnerInvoiceServiceImpl {
    /// 构建实现（对应 Java 构造器 `PartnerInvoiceServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }

    /// 构建查询参数。
    fn query_param(key: &str, value: &str) -> String {
        format!("{}={}", key, urlencoding(value))
    }

    /// 追加查询参数。
    fn append_query(url: &str, key: &str, value: Option<&str>) -> String {
        match value {
            Some(v) => {
                let separator = if url.contains('?') { "&" } else { "?" };
                format!("{}{}{}", url, separator, Self::query_param(key, v))
            }
            None => url.to_string(),
        }
    }
}

#[async_trait]
impl PartnerInvoiceService for PartnerInvoiceServiceImpl {
    async fn get_invite_url_by_mch_id(
        &self,
        sub_mch_id: Option<&str>,
    ) -> Result<InviteUrlResult, WxErrorException> {
        let mut request = InviteUrlRequest::default();
        request.sub_mchid = sub_mch_id.map(str::to_string);
        self.get_invite_url(&request).await
    }

    async fn get_invite_url(
        &self,
        request: &InviteUrlRequest,
    ) -> Result<InviteUrlResult, WxErrorException> {
        let svc = self.svc()?;
        let base = format!("{}{}", svc.get_pay_base_url(), INVITE_URL_PATH);
        let mut url = base;
        if let Some(ref sub_mchid) = request.sub_mchid {
            url = format!("{}?{}", url, Self::query_param("sub_mchid", sub_mchid));
        }
        url = Self::append_query(&url, "operation_type", request.operation_type.as_deref());
        url = Self::append_query(&url, "fapiao_mode", request.fapiao_mode.as_deref());
        if let Some(ref list) = request.fapiao_ability_type_list {
            if !list.is_empty() {
                let joined = list.join(",");
                let separator = if url.contains('?') { "&" } else { "?" };
                url = format!(
                    "{}{}fapiao_ability_type_list={}",
                    url,
                    separator,
                    urlencoding(&joined)
                );
            }
        }
        url = Self::append_query(&url, "invite_channel", request.invite_channel.as_deref());
        url = Self::append_query(&url, "operate_user", request.operate_user.as_deref());
        url = Self::append_query(&url, "invite_code", request.invite_code.as_deref());
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn issue_general_invoice(
        &self,
        request: &GeneralInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!("{}{}", svc.get_pay_base_url(), ISSUE_GENERAL_PATH);
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn get_invoice(
        &self,
        fapiao_apply_id: &str,
        sub_mch_id: &str,
        fapiao_id: Option<&str>,
    ) -> Result<InvoiceResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}{}{}?sub_mchid={}",
            svc.get_pay_base_url(),
            FAPIAO_APPLICATIONS_PATH,
            urlencoding(fapiao_apply_id),
            urlencoding(sub_mch_id)
        );
        if let Some(fid) = fapiao_id {
            url = format!("{}&fapiao_id={}", url, urlencoding(fid));
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn reverse_invoice(
        &self,
        request: &ReverseInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let fapiao_apply_id = request
            .fapiao_apply_id
            .as_deref()
            .ok_or_else(|| impl_utils::runtime("fapiao_apply_id 不能为空"))?;
        let url = format!(
            "{}{}{}/reverse",
            svc.get_pay_base_url(),
            FAPIAO_APPLICATIONS_PATH,
            urlencoding(fapiao_apply_id)
        );
        // 构造 body，移除 fapiao_apply_id（仅用于 URL 构造）
        let mut body_value =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        if let Some(obj) = body_value.as_object_mut() {
            obj.remove("fapiao_apply_id");
        }
        let body =
            serde_json::to_string(&body_value).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn get_invoice_file_download_info(
        &self,
        fapiao_apply_id: &str,
        sub_mch_id: &str,
        fapiao_id: Option<&str>,
    ) -> Result<InvoiceFileResult, WxErrorException> {
        let svc = self.svc()?;
        let mut url = format!(
            "{}{}{}/fapiao-files?sub_mchid={}",
            svc.get_pay_base_url(),
            FAPIAO_APPLICATIONS_PATH,
            urlencoding(fapiao_apply_id),
            urlencoding(sub_mch_id)
        );
        if let Some(fid) = fapiao_id {
            url = format!("{}&fapiao_id={}", url, urlencoding(fid));
        }
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_sub_merchant_invoice_status(
        &self,
        sub_mch_id: &str,
    ) -> Result<SubMerchantInvoiceStatus, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/merchant/{}/check-status",
            svc.get_pay_base_url(),
            urlencoding(sub_mch_id)
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn create_card_template(
        &self,
        request: &CardTemplateRequest,
    ) -> Result<CardTemplateResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/card-template",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn update_development_config(
        &self,
        request: &DevelopmentConfigRequest,
    ) -> Result<DevelopmentConfigResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/merchant/development-config",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.patch_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_user_title_url(
        &self,
        request: &TitleUrlRequest,
    ) -> Result<TitleUrlResult, WxErrorException> {
        let svc = self.svc()?;
        let base = format!(
            "{}/v3/new-tax-control-fapiao/user-title/title-url",
            svc.get_pay_base_url()
        );
        let mut url = format!(
            "{}?{}&{}&{}&{}&{}&{}",
            base,
            Self::query_param(
                "sub_mchid",
                request.sub_mchid.as_deref().unwrap_or_default()
            ),
            Self::query_param(
                "fapiao_apply_id",
                request.fapiao_apply_id.as_deref().unwrap_or_default()
            ),
            Self::query_param("source", request.source.as_deref().unwrap_or_default()),
            Self::query_param("appid", request.appid.as_deref().unwrap_or_default()),
            Self::query_param("openid", request.openid.as_deref().unwrap_or_default()),
            Self::query_param(
                "total_amount",
                &request
                    .total_amount
                    .map(|v| v.to_string())
                    .unwrap_or_default()
            ),
        );
        url = Self::append_query(&url, "seller_name", request.seller_name.as_deref());
        url = Self::append_query(
            &url,
            "show_phone_cell",
            request
                .show_phone_cell
                .map(|v| if v { "true" } else { "false" }),
        );
        url = Self::append_query(
            &url,
            "must_input_phone",
            request
                .must_input_phone
                .map(|v| if v { "true" } else { "false" }),
        );
        url = Self::append_query(
            &url,
            "show_email_cell",
            request
                .show_email_cell
                .map(|v| if v { "true" } else { "false" }),
        );
        url = Self::append_query(
            &url,
            "must_input_email",
            request
                .must_input_email
                .map(|v| if v { "true" } else { "false" }),
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn get_user_title(
        &self,
        sub_mch_id: &str,
        scene: &str,
        fapiao_apply_id: &str,
    ) -> Result<BuyerInformation, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/user-title?{}&{}&{}",
            svc.get_pay_base_url(),
            Self::query_param("sub_mchid", sub_mch_id),
            Self::query_param("scene", scene),
            Self::query_param("fapiao_apply_id", fapiao_apply_id),
        );
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn issue_real_estate_leasing_invoice(
        &self,
        request: &IndustryInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/fapiao-applications/real-estate-leasing",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn issue_refined_oil_invoice(
        &self,
        request: &IndustryInvoiceRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}/v3/new-tax-control-fapiao/fapiao-applications/issue-refined-oil",
            svc.get_pay_base_url()
        );
        let body =
            serde_json::to_string(&request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn insert_cards(&self, request: &InsertCardRequest) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let fapiao_apply_id = request
            .fapiao_apply_id
            .as_deref()
            .ok_or_else(|| impl_utils::runtime("fapiao_apply_id 不能为空"))?;
        let url = format!(
            "{}{}{}/insert-cards",
            svc.get_pay_base_url(),
            FAPIAO_APPLICATIONS_PATH,
            urlencoding(fapiao_apply_id)
        );
        // 构造 body，移除 fapiao_apply_id（仅用于 URL 构造）
        let mut body_value =
            serde_json::to_value(request).map_err(|e| impl_utils::runtime(e.to_string()))?;
        if let Some(obj) = body_value.as_object_mut() {
            obj.remove("fapiao_apply_id");
        }
        let body =
            serde_json::to_string(&body_value).map_err(|e| impl_utils::runtime(e.to_string()))?;
        svc.post_v3(&url, &body).await?;
        Ok(())
    }

    async fn list_invite_merchants(
        &self,
        query: &InviteMerchantQuery,
    ) -> Result<InviteMerchantResult, WxErrorException> {
        let svc = self.svc()?;
        let base = format!(
            "{}/v3/new-tax-control-fapiao/fapiaomerchant/listspinvitemchinfo",
            svc.get_pay_base_url()
        );
        let mut url = format!(
            "{}?{}&{}&{}&{}&{}",
            base,
            Self::query_param(
                "query_time_start",
                query.query_time_start.as_deref().unwrap_or_default()
            ),
            Self::query_param(
                "query_time_end",
                query.query_time_end.as_deref().unwrap_or_default()
            ),
            Self::query_param(
                "offset",
                &query.offset.map(|v| v.to_string()).unwrap_or_default()
            ),
            Self::query_param(
                "limit",
                &query.limit.map(|v| v.to_string()).unwrap_or_default()
            ),
            Self::query_param(
                "mch_invite_status",
                query.mch_invite_status.as_deref().unwrap_or_default()
            ),
        );
        url = Self::append_query(&url, "invite_code", query.invite_code.as_deref());
        let response = svc.get_v3(&url).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }

    async fn upload_invoice_file(
        &self,
        request: &InvoiceFileUploadRequest,
    ) -> Result<InvoiceFileUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let url = format!(
            "{}{}upload-fapiao-file",
            svc.get_pay_base_url(),
            FAPIAO_APPLICATIONS_PATH
        );
        // 构造 meta JSON（对应 Java WechatPayUploadHttpPost 的 meta 部分）
        let meta = json!({
            "sub_mchid": request.sub_mchid,
            "file_type": request.file_type.as_deref().unwrap_or("PDF"),
            "digest_alogrithm": request.digest_alogrithm.as_deref().unwrap_or("SM3"),
            "digest": request.digest,
        });
        let body = serde_json::to_string(&meta).map_err(|e| impl_utils::runtime(e.to_string()))?;
        let response = svc.post_v3(&url, &body).await?;
        serde_json::from_str(&response).map_err(|e| impl_utils::runtime(e.to_string()))
    }
}
