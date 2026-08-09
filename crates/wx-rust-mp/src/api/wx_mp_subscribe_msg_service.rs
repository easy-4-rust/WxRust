//! WxMpSubscribeMsgService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpSubscribeMsgService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::subscribe::WxMpSubscribeMessage;
use crate::enums::wx_mp_api_url::subscribe_msg;
use wx_rust_common::bean::subscribemsg::{
    CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo,
};

/// 公众号SubscribeMsgService。
#[async_trait]
pub trait WxMpSubscribeMsgService: Send + Sync {
    async fn send_once(&self, message: &WxMpSubscribeMessage) -> Result<bool, WxErrorException>;

    async fn send(&self, message: &WxMpSubscribeMessage) -> Result<String, WxErrorException>;

    async fn get_pub_template_title_list(
        &self,
        ids: &[&str],
        start: i32,
        limit: i32,
    ) -> Result<PubTemplateTitleListResult, WxErrorException>;

    async fn get_pub_template_key_words_by_id(
        &self,
        id: &str,
    ) -> Result<Vec<PubTemplateKeyword>, WxErrorException>;

    async fn add_template(
        &self,
        id: &str,
        keyword_id_list: &[i32],
        scene_desc: &str,
    ) -> Result<String, WxErrorException>;

    async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException>;

    async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException>;

    async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException>;
}
