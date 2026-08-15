//! WxMpGuideService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpGuideService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::guide::{
    WxMpAddGuideAutoReply, WxMpGuideAcctConfig, WxMpGuideConfig, WxMpGuideGroupInfoList,
    WxMpGuideInfo, WxMpGuideList, WxMpGuideMsgList,
};

/// 公众号GuideService。
#[async_trait]
pub trait WxMpGuideService: Send + Sync {
    async fn add_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException>;

    async fn update_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException>;

    async fn get_guide(
        &self,
        account: &str,
        openid: &str,
    ) -> Result<WxMpGuideInfo, WxErrorException>;

    async fn del_guide(&self, account: &str, openid: &str) -> Result<(), WxErrorException>;

    async fn list_guide(&self, page: i32, num: i32) -> Result<WxMpGuideList, WxErrorException>;

    async fn create_guide_qr_code(
        &self,
        account: &str,
        openid: &str,
        qrcode_info: &str,
    ) -> Result<String, WxErrorException>;

    async fn get_guide_chat_record(
        &self,
        account: &str,
        openid: &str,
        client_openid: &str,
        begin_time: i64,
        end_time: i64,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideMsgList, WxErrorException>;

    async fn set_guide_config(
        &self,
        account: &str,
        openid: &str,
        is_delete: bool,
        guide_fast_reply_list: &[String],
        guide_auto_reply: &WxMpAddGuideAutoReply,
        guide_auto_reply_plus: &WxMpAddGuideAutoReply,
    ) -> Result<(), WxErrorException>;

    async fn get_guide_config(
        &self,
        account: &str,
        openid: &str,
    ) -> Result<WxMpGuideConfig, WxErrorException>;

    async fn set_guide_acct_config(
        &self,
        is_delete: bool,
        black_keyword: &[String],
        guide_auto_reply: &str,
    ) -> Result<(), WxErrorException>;

    async fn get_guide_acct_config(&self) -> Result<WxMpGuideAcctConfig, WxErrorException>;

    async fn new_guide_group(&self, name: &str) -> Result<i64, WxErrorException>;

    async fn get_guide_group_list(
        &self,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideGroupInfoList, WxErrorException>;
}
