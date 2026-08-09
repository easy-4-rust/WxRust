//! WxMpGuideTagService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpGuideTagService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::guide::{
    WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp, WxMpGuideCardMaterialInfo,
    WxMpGuideImgMaterialInfoList, WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo,
    WxMpGuideTagInfo, WxMpGuideWordMaterialInfoList,
};
use crate::enums::wx_mp_api_url::guide;

/// 公众号GuideTagService。
#[async_trait]
pub trait WxMpGuideTagService: Send + Sync {
    async fn new_guide_tag_option(
        &self,
        tag_name: &str,
        values: &[String],
    ) -> Result<(), WxErrorException>;

    async fn del_guide_tag_option(&self, tag_name: &str) -> Result<(), WxErrorException>;

    async fn add_guide_tag_option(
        &self,
        tag_name: &str,
        values: &[String],
    ) -> Result<(), WxErrorException>;

    async fn get_guide_tag_option(&self) -> Result<Vec<WxMpGuideTagInfo>, WxErrorException>;

    async fn add_guide_buyer_tag(
        &self,
        account: &str,
        openid: &str,
        value: &str,
        user_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>;

    async fn get_guide_buyer_tag(
        &self,
        account: &str,
        openid: &str,
        user_openid: &str,
        is_exclude: bool,
    ) -> Result<Vec<String>, WxErrorException>;

    async fn query_guide_buyer_by_tag(
        &self,
        account: &str,
        openid: &str,
        push_count: i32,
        values: &[String],
    ) -> Result<Vec<String>, WxErrorException>;
}
