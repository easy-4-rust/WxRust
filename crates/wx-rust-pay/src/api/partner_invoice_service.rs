//! 微信支付服务商电子发票 API。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.PartnerInvoiceService`。
//!
//! 产品介绍: <https://pay.weixin.qq.com/doc/v3/partner/4015941495>

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

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

/// 微信支付服务商电子发票服务（对应 Java `PartnerInvoiceService`）。
#[async_trait]
pub trait PartnerInvoiceService: Send + Sync {
    /// 获取开通服务商电子发票能力邀请链接。
    ///
    /// 对应 Java: `PartnerInvoiceService#getInviteUrl(String)`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015941495>
    ///
    /// # 参数
    /// - `sub_mch_id`: 可选，指定要邀请开通能力的子商户号
    async fn get_invite_url_by_mch_id(
        &self,
        sub_mch_id: Option<&str>,
    ) -> Result<InviteUrlResult, WxErrorException>;

    /// 获取开通服务商电子发票能力邀请链接（完整请求参数）。
    ///
    /// 对应 Java: `PartnerInvoiceService#getInviteUrl(InviteUrlRequest)`
    async fn get_invite_url(
        &self,
        request: &InviteUrlRequest,
    ) -> Result<InviteUrlResult, WxErrorException>;

    /// 开具通用行业电子发票。
    ///
    /// 对应 Java: `PartnerInvoiceService#issueGeneralInvoice`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4015792574>
    async fn issue_general_invoice(
        &self,
        request: &GeneralInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    /// 查询电子发票。
    ///
    /// 对应 Java: `PartnerInvoiceService#getInvoice`
    ///
    /// # 参数
    /// - `fapiao_apply_id`: 开票申请单号
    /// - `sub_mch_id`: 子商户号
    /// - `fapiao_id`: 可选，商户发票单号
    async fn get_invoice(
        &self,
        fapiao_apply_id: &str,
        sub_mch_id: &str,
        fapiao_id: Option<&str>,
    ) -> Result<InvoiceResult, WxErrorException>;

    /// 冲红电子发票。
    ///
    /// 对应 Java: `PartnerInvoiceService#reverseInvoice`
    async fn reverse_invoice(
        &self,
        request: &ReverseInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    /// 获取发票文件下载信息。
    ///
    /// 对应 Java: `PartnerInvoiceService#getInvoiceFileDownloadInfo`
    ///
    /// # 参数
    /// - `fapiao_apply_id`: 开票申请单号
    /// - `sub_mch_id`: 子商户号
    /// - `fapiao_id`: 可选，商户发票单号
    async fn get_invoice_file_download_info(
        &self,
        fapiao_apply_id: &str,
        sub_mch_id: &str,
        fapiao_id: Option<&str>,
    ) -> Result<InvoiceFileResult, WxErrorException>;

    /// 检查子商户开票功能状态。
    ///
    /// 对应 Java: `PartnerInvoiceService#getSubMerchantInvoiceStatus`
    async fn get_sub_merchant_invoice_status(
        &self,
        sub_mch_id: &str,
    ) -> Result<SubMerchantInvoiceStatus, WxErrorException>;

    /// 创建电子发票卡券模板。
    ///
    /// 对应 Java: `PartnerInvoiceService#createCardTemplate`
    async fn create_card_template(
        &self,
        request: &CardTemplateRequest,
    ) -> Result<CardTemplateResult, WxErrorException>;

    /// 更新开发配置。
    ///
    /// 对应 Java: `PartnerInvoiceService#updateDevelopmentConfig`
    async fn update_development_config(
        &self,
        request: &DevelopmentConfigRequest,
    ) -> Result<DevelopmentConfigResult, WxErrorException>;

    /// 获取用户抬头填写链接。
    ///
    /// 对应 Java: `PartnerInvoiceService#getUserTitleUrl`
    async fn get_user_title_url(
        &self,
        request: &TitleUrlRequest,
    ) -> Result<TitleUrlResult, WxErrorException>;

    /// 获取用户抬头信息。
    ///
    /// 对应 Java: `PartnerInvoiceService#getUserTitle`
    async fn get_user_title(
        &self,
        sub_mch_id: &str,
        scene: &str,
        fapiao_apply_id: &str,
    ) -> Result<BuyerInformation, WxErrorException>;

    /// 开具不动产租赁行业电子发票。
    ///
    /// 对应 Java: `PartnerInvoiceService#issueRealEstateLeasingInvoice`
    async fn issue_real_estate_leasing_invoice(
        &self,
        request: &IndustryInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    /// 开具成品油行业电子发票。
    ///
    /// 对应 Java: `PartnerInvoiceService#issueRefinedOilInvoice`
    async fn issue_refined_oil_invoice(
        &self,
        request: &IndustryInvoiceRequest,
    ) -> Result<(), WxErrorException>;

    /// 将电子发票插入微信用户卡包。
    ///
    /// 对应 Java: `PartnerInvoiceService#insertCards`
    async fn insert_cards(&self, request: &InsertCardRequest) -> Result<(), WxErrorException>;

    /// 查询服务商邀请开通电子发票能力的商户。
    ///
    /// 对应 Java: `PartnerInvoiceService#listInviteMerchants`
    async fn list_invite_merchants(
        &self,
        query: &InviteMerchantQuery,
    ) -> Result<InviteMerchantResult, WxErrorException>;

    /// 上传电子发票 PDF 文件。
    ///
    /// 对应 Java: `PartnerInvoiceService#uploadInvoiceFile`
    async fn upload_invoice_file(
        &self,
        request: &InvoiceFileUploadRequest,
    ) -> Result<InvoiceFileUploadResult, WxErrorException>;
}
