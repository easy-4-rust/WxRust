//! WxMpMemberCardService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpMemberCardService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::card::CardUpdateResult;
use crate::bean::card::membercard::{
    MemberCardActivateUserFormRequest, MemberCardActivateUserFormResult, MemberCardUpdateRequest,
    WxMpMemberCardActivateTempInfoResult, WxMpMemberCardActivatedMessage,
    WxMpMemberCardCreateMessage, WxMpMemberCardUpdateMessage, WxMpMemberCardUpdateResult,
    WxMpMemberCardUserInfoResult,
};

/// 公众号MemberCardService。
#[async_trait]
pub trait WxMpMemberCardService: Send + Sync {
    async fn create_member_card(
        &self,
        message: &WxMpMemberCardCreateMessage,
    ) -> Result<String, WxErrorException>;

    async fn activate_member_card(
        &self,
        message: &WxMpMemberCardActivatedMessage,
    ) -> Result<String, WxErrorException>;

    async fn get_user_info(
        &self,
        card_id: &str,
        code: &str,
    ) -> Result<WxMpMemberCardUserInfoResult, WxErrorException>;

    async fn update_user_member_card(
        &self,
        message: &WxMpMemberCardUpdateMessage,
    ) -> Result<WxMpMemberCardUpdateResult, WxErrorException>;

    async fn set_activate_user_form(
        &self,
        request: &MemberCardActivateUserFormRequest,
    ) -> Result<MemberCardActivateUserFormResult, WxErrorException>;

    async fn update_card_info(
        &self,
        request: &MemberCardUpdateRequest,
    ) -> Result<CardUpdateResult, WxErrorException>;

    async fn get_activate_temp_info(
        &self,
        activate_ticket: &str,
    ) -> Result<WxMpMemberCardActivateTempInfoResult, WxErrorException>;
}
