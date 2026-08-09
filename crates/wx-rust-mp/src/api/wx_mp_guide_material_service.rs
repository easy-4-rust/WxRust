//! WxMpGuideMaterialService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpGuideMaterialService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::guide::{
    WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp, WxMpGuideCardMaterialInfo,
    WxMpGuideImgMaterialInfoList, WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo,
    WxMpGuideTagInfo, WxMpGuideWordMaterialInfoList,
};
use crate::enums::wx_mp_api_url::guide;

/// 公众号GuideMaterialService。
#[async_trait]
pub trait WxMpGuideMaterialService: Send + Sync {
    async fn set_guide_card_material(
        &self,
        media_id: &str,
        r#type: i32,
        title: &str,
        path: &str,
        app_id: &str,
    ) -> Result<(), WxErrorException>;

    async fn get_guide_card_material(
        &self,
        r#type: i32,
    ) -> Result<Vec<WxMpGuideCardMaterialInfo>, WxErrorException>;

    async fn del_guide_card_material(
        &self,
        r#type: i32,
        title: &str,
        path: &str,
        app_id: &str,
    ) -> Result<(), WxErrorException>;

    async fn set_guide_image_material(
        &self,
        media_id: &str,
        r#type: i32,
    ) -> Result<(), WxErrorException>;

    async fn get_guide_image_material(
        &self,
        r#type: i32,
        start: i32,
        num: i32,
    ) -> Result<WxMpGuideImgMaterialInfoList, WxErrorException>;

    async fn del_guide_image_material(
        &self,
        r#type: i32,
        pic_url: &str,
    ) -> Result<(), WxErrorException>;

    async fn set_guide_word_material(
        &self,
        r#type: i32,
        word: &str,
    ) -> Result<(), WxErrorException>;

    async fn get_guide_word_material(
        &self,
        r#type: i32,
        start: i32,
        num: i32,
    ) -> Result<WxMpGuideWordMaterialInfoList, WxErrorException>;

    async fn del_guide_word_material(
        &self,
        r#type: i32,
        word: &str,
    ) -> Result<(), WxErrorException>;
}
