//! WxMpSubscribeMsgService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpSubscribeMsgServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpService, WxMpSubscribeMsgService};
use crate::bean::subscribe::WxMpSubscribeMessage;
use crate::enums::wx_mp_api_url::subscribe_msg;
use wx_rust_common::bean::subscribemsg::{
    CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo,
};

/// 公众号SubscribeMsgService实现。
pub struct WxMpSubscribeMsgServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpSubscribeMsgServiceImpl {
    /// 构建 公众号SubscribeMsgService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 校验响应 errcode 是否为 0。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value
            .get("errcode")
            .map(|v| v.as_i64() == Some(0))
            .unwrap_or(false))
    }
}

#[async_trait]
impl WxMpSubscribeMsgService for WxMpSubscribeMsgServiceImpl {
    async fn send_once(&self, message: &WxMpSubscribeMessage) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = message.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&subscribe_msg::send_once(config.as_ref()), &body)
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn send(&self, message: &WxMpSubscribeMessage) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = message.to_json().map_err(WxErrorException::Serde)?;
        let response = svc
            .post(&subscribe_msg::send(config.as_ref()), &body)
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("msgid")
            .map(|v| v.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "msgid 缺失"))
    }

    async fn get_pub_template_title_list(
        &self,
        ids: &[&str],
        start: i32,
        limit: i32,
    ) -> Result<PubTemplateTitleListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut query = format!("start={start}&limit={limit}");
        for id in ids {
            query.push_str(&format!("&ids={id}"));
        }
        let response = svc
            .get(
                &subscribe_msg::get_pub_template_title_list(config.as_ref()),
                &query,
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_pub_template_key_words_by_id(
        &self,
        id: &str,
    ) -> Result<Vec<PubTemplateKeyword>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let query = format!("tid={id}");
        let response = svc
            .get(
                &subscribe_msg::get_pub_template_key_words_by_id(config.as_ref()),
                &query,
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("data")
            .ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_template(
        &self,
        id: &str,
        keyword_id_list: &[i32],
        scene_desc: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body =
            serde_json::json!({"tid": id, "kidList": keyword_id_list, "sceneDesc": scene_desc});
        let response = svc
            .post(
                &subscribe_msg::template_add(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("priTmplId")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "priTmplId 缺失"))
    }

    async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&subscribe_msg::template_list(config.as_ref()), "")
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("data")
            .ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"priTmplId": template_id});
        let response = svc
            .post(
                &subscribe_msg::template_del(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::err_code_is_zero(&response)
    }

    async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .get(&subscribe_msg::get_category(config.as_ref()), "")
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("data")
            .ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
