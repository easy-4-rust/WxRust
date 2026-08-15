//! WxMpGuideMaterialService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpGuideMaterialServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpGuideMaterialService, WxMpService};
use crate::bean::guide::{
    WxMpGuideCardMaterialInfo, WxMpGuideImgMaterialInfoList, WxMpGuideWordMaterialInfoList,
};
use crate::enums::wx_mp_api_url::guide;

pub struct WxMpGuideMaterialServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpGuideMaterialServiceImpl {
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpGuideMaterialService for WxMpGuideMaterialServiceImpl {
    async fn set_guide_card_material(
        &self,
        media_id: &str,
        r#type: i32,
        title: &str,
        path: &str,
        app_id: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "media_id": media_id, "title": title, "path": path, "appid": app_id});
        svc.post(
            &guide::set_guide_card_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_card_material(
        &self,
        r#type: i32,
    ) -> Result<Vec<WxMpGuideCardMaterialInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type});
        let response = svc
            .post(
                &guide::get_guide_card_material(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("card_material_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "card_material_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn del_guide_card_material(
        &self,
        r#type: i32,
        title: &str,
        path: &str,
        app_id: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::json!({"type": r#type, "title": title, "path": path, "appid": app_id});
        svc.post(
            &guide::del_guide_card_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn set_guide_image_material(
        &self,
        media_id: &str,
        r#type: i32,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "media_id": media_id});
        svc.post(
            &guide::set_guide_image_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_image_material(
        &self,
        r#type: i32,
        start: i32,
        num: i32,
    ) -> Result<WxMpGuideImgMaterialInfoList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "start": start, "num": num});
        let response = svc
            .post(
                &guide::get_guide_image_material(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpGuideImgMaterialInfoList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn del_guide_image_material(
        &self,
        r#type: i32,
        pic_url: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "pic_url": pic_url});
        svc.post(
            &guide::del_guide_image_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn set_guide_word_material(
        &self,
        r#type: i32,
        word: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "word": word});
        svc.post(
            &guide::set_guide_word_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_word_material(
        &self,
        r#type: i32,
        start: i32,
        num: i32,
    ) -> Result<WxMpGuideWordMaterialInfoList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "start": start, "num": num});
        let response = svc
            .post(
                &guide::get_guide_word_material(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpGuideWordMaterialInfoList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn del_guide_word_material(
        &self,
        r#type: i32,
        word: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"type": r#type, "word": word});
        svc.post(
            &guide::del_guide_word_material(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }
}
