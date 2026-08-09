//! WxMpGuideBuyerService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpGuideBuyerService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::guide::{
    WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp, WxMpGuideCardMaterialInfo,
    WxMpGuideImgMaterialInfoList, WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo,
    WxMpGuideTagInfo, WxMpGuideWordMaterialInfoList,
};
use crate::enums::wx_mp_api_url::guide;

/// 公众号GuideBuyerService。
#[async_trait]
pub trait WxMpGuideBuyerService: Send + Sync {
    async fn add_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        infos: &[WxMpAddGuideBuyerInfo],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>;

    async fn del_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        buyer_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>;

    async fn get_guide_buyer_relation_list(
        &self,
        account: &str,
        openid: &str,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideBuyerInfoList, WxErrorException>;

    async fn rebind_guide_acct_for_buyer(
        &self,
        old_account: &str,
        old_openid: &str,
        account: &str,
        openid: &str,
        buyer_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>;

    async fn update_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        user_openid: &str,
        nickname: &str,
    ) -> Result<(), WxErrorException>;
}
