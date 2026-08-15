//! WxMpMerchantInvoiceService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMerchantInvoiceService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::invoice::merchant::{
    ClearOutInvoiceRequest, InvoiceAuthDataRequest, InvoiceAuthDataResult, InvoiceAuthPageRequest,
    InvoiceAuthPageResult, InvoiceAuthPageSetting, InvoiceRejectRequest, InvoiceResult,
    MakeOutInvoiceRequest, MerchantContactInfo, MerchantInvoicePlatformInfo,
};

/// 公众号MerchantInvoiceService。
#[async_trait]
pub trait WxMpMerchantInvoiceService: Send + Sync {
    async fn get_auth_page_url(
        &self,
        params: &InvoiceAuthPageRequest,
    ) -> Result<InvoiceAuthPageResult, WxErrorException>;

    async fn get_auth_data(
        &self,
        params: &InvoiceAuthDataRequest,
    ) -> Result<InvoiceAuthDataResult, WxErrorException>;

    async fn reject_invoice(&self, params: &InvoiceRejectRequest) -> Result<(), WxErrorException>;

    async fn make_out_invoice(
        &self,
        params: &MakeOutInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    async fn clear_out_invoice(
        &self,
        params: &ClearOutInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    async fn query_invoice_info(
        &self,
        fpqqlsh: &str,
        nsrsbh: &str,
    ) -> Result<InvoiceResult, WxErrorException>;

    async fn set_merchant_contact_info(
        &self,
        contact: &MerchantContactInfo,
    ) -> Result<(), WxErrorException>;

    async fn get_merchant_contact_info(&self) -> Result<MerchantContactInfo, WxErrorException>;

    async fn set_auth_page_setting(
        &self,
        setting: &InvoiceAuthPageSetting,
    ) -> Result<(), WxErrorException>;

    async fn get_auth_page_setting(&self) -> Result<InvoiceAuthPageSetting, WxErrorException>;

    async fn set_merchant_invoice_platform(
        &self,
        info: &MerchantInvoicePlatformInfo,
    ) -> Result<(), WxErrorException>;

    async fn get_merchant_invoice_platform(
        &self,
        info: &MerchantInvoicePlatformInfo,
    ) -> Result<MerchantInvoicePlatformInfo, WxErrorException>;
}
