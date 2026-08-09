//! WxMpGuideService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpGuideServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpGuideService, WxMpService};
use crate::bean::guide::{
    WxMpAddGuideAutoReply, WxMpGuideAcctConfig, WxMpGuideConfig, WxMpGuideGroupInfoList,
    WxMpGuideInfo, WxMpGuideList, WxMpGuideMsgList,
};
use crate::enums::wx_mp_api_url::guide;

/// 公众号GuideService实现。
pub struct WxMpGuideServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpGuideServiceImpl {
    /// 构建 公众号GuideService。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpGuideService for WxMpGuideServiceImpl {
    async fn add_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::to_string(guide_info)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&guide::add_guide(config.as_ref()), &body).await?;
        Ok(())
    }

    async fn update_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::to_string(guide_info)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&guide::update_guide(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    async fn get_guide(
        &self,
        account: &str,
        openid: &str,
    ) -> Result<WxMpGuideInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        let response = svc
            .post(&guide::get_guide(config.as_ref()), &body.to_string())
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let guide = value
            .get("guide_info")
            .ok_or_else(|| WxErrorException::from_code(-99, "guide_info 缺失"))?;
        serde_json::from_value(guide.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn del_guide(&self, account: &str, openid: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        svc.post(&guide::del_guide(config.as_ref()), &body.to_string())
            .await?;
        Ok(())
    }

    async fn list_guide(&self, page: i32, num: i32) -> Result<WxMpGuideList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"page": page, "num": num});
        let response = svc
            .post(&guide::list_guide(config.as_ref()), &body.to_string())
            .await?;
        WxMpGuideList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn create_guide_qr_code(
        &self,
        account: &str,
        openid: &str,
        qrcode_info: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "qrcode_info": qrcode_info});
        let response = svc
            .post(&guide::create_qr_code(config.as_ref()), &body.to_string())
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("qrcode_url")
            .and_then(|v| v.as_str())
            .map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "qrcode_url 缺失"))
    }

    async fn get_guide_chat_record(
        &self,
        account: &str,
        openid: &str,
        client_openid: &str,
        begin_time: i64,
        end_time: i64,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideMsgList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": client_openid, "begin_time": begin_time, "end_time": end_time, "page": page, "num": num});
        let response = svc
            .post(
                &guide::get_guide_chat_record(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("guide_msg_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "guide_msg_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_guide_config(
        &self,
        account: &str,
        openid: &str,
        is_delete: bool,
        guide_fast_reply_list: &[String],
        guide_auto_reply: &WxMpAddGuideAutoReply,
        guide_auto_reply_plus: &WxMpAddGuideAutoReply,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert("guide_account".into(), serde_json::json!(account));
        body.insert("guide_openid".into(), serde_json::json!(openid));
        body.insert(
            "is_delete".into(),
            serde_json::json!(if is_delete { 1 } else { 0 }),
        );
        body.insert(
            "guide_fast_reply_list".into(),
            serde_json::json!(guide_fast_reply_list),
        );
        body.insert(
            "guide_auto_reply".into(),
            serde_json::to_value(guide_auto_reply).unwrap_or_default(),
        );
        body.insert(
            "guide_auto_reply_plus".into(),
            serde_json::to_value(guide_auto_reply_plus).unwrap_or_default(),
        );
        svc.post(
            &guide::set_guide_config(config.as_ref()),
            &serde_json::Value::Object(body).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_config(
        &self,
        account: &str,
        openid: &str,
    ) -> Result<WxMpGuideConfig, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        let response = svc
            .post(&guide::get_guide_config(config.as_ref()), &body.to_string())
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = value
            .get("guide_config")
            .ok_or_else(|| WxErrorException::from_code(-99, "guide_config 缺失"))?;
        serde_json::from_value(config.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn set_guide_acct_config(
        &self,
        is_delete: bool,
        black_keyword: &[String],
        guide_auto_reply: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let mut body = serde_json::Map::new();
        body.insert(
            "is_delete".into(),
            serde_json::json!(if is_delete { 1 } else { 0 }),
        );
        body.insert("black_keyword".into(), serde_json::json!(black_keyword));
        body.insert(
            "guide_auto_reply".into(),
            serde_json::json!(guide_auto_reply),
        );
        svc.post(
            &guide::set_guide_acct_config(config.as_ref()),
            &serde_json::Value::Object(body).to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_acct_config(&self) -> Result<WxMpGuideAcctConfig, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .post(&guide::get_guide_acct_config(config.as_ref()), "{}")
            .await?;
        WxMpGuideAcctConfig::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn new_guide_group(&self, name: &str) -> Result<i64, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"group_name": name});
        let response = svc
            .post(&guide::new_guide_group(config.as_ref()), &body.to_string())
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("group_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "group_id 缺失"))
    }

    async fn get_guide_group_list(
        &self,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideGroupInfoList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"page": page, "num": num});
        let response = svc
            .post(
                &guide::get_guide_group_list(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("group_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "group_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
