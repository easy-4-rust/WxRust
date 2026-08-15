//! WxMpCardService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpCardService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::card::{
    WxMpCardCreateRequest, WxMpCardCreateResult, WxMpCardDeleteResult,
    WxMpCardLandingPageCreateRequest, WxMpCardLandingPageCreateResult, WxMpCardQrcodeCreateResult,
    WxMpCardResult,
};
use wx_rust_common::bean::WxCardApiSignature;

/// 公众号CardService。
#[async_trait]
pub trait WxMpCardService: Send + Sync {
    async fn get_card_api_ticket(&self, force_refresh: bool) -> Result<String, WxErrorException>;

    async fn create_card_api_signature(
        &self,
        optional_sign_param: &[&str],
    ) -> Result<WxCardApiSignature, WxErrorException>;

    async fn decrypt_card_code(&self, encrypt_code: &str) -> Result<String, WxErrorException>;

    async fn query_card_code(
        &self,
        card_id: &str,
        code: &str,
        check_consume: bool,
    ) -> Result<WxMpCardResult, WxErrorException>;

    async fn consume_card_code(&self, code: &str) -> Result<String, WxErrorException>;

    async fn mark_card_code(
        &self,
        code: &str,
        card_id: &str,
        open_id: &str,
        is_mark: bool,
    ) -> Result<(), WxErrorException>;

    async fn get_card_detail(&self, card_id: &str) -> Result<String, WxErrorException>;

    async fn add_test_white_list(&self, openid: &str) -> Result<String, WxErrorException>;

    async fn create_card(
        &self,
        request: &WxMpCardCreateRequest,
    ) -> Result<WxMpCardCreateResult, WxErrorException>;

    async fn delete_card(&self, card_id: &str) -> Result<WxMpCardDeleteResult, WxErrorException>;

    async fn create_landing_page(
        &self,
        request: &WxMpCardLandingPageCreateRequest,
    ) -> Result<WxMpCardLandingPageCreateResult, WxErrorException>;

    async fn create_qrcode_card(
        &self,
        card_id: &str,
        outer_str: &str,
        expires_in: Option<i32>,
    ) -> Result<WxMpCardQrcodeCreateResult, WxErrorException>;
}
