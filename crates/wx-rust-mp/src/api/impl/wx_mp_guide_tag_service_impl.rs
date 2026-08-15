//! WxMpGuideTagService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpGuideTagServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpGuideTagService, WxMpService};
use crate::bean::guide::{WxMpGuideBuyerResp, WxMpGuideTagInfo};
use crate::enums::wx_mp_api_url::guide;

pub struct WxMpGuideTagServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpGuideTagServiceImpl {
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 解析买家操作响应列表（对应 Java `get("buyer_resp")`）。
    fn parse_buyer_resp_list(response: &str) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let value: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("buyer_resp")
            .ok_or_else(|| WxErrorException::from_code(-99, "buyer_resp 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxMpGuideTagService for WxMpGuideTagServiceImpl {
    async fn new_guide_tag_option(
        &self,
        tag_name: &str,
        values: &[String],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag_name": tag_name, "tag_values": values});
        svc.post(
            &guide::new_guide_tag_option(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn del_guide_tag_option(&self, tag_name: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag_name": tag_name});
        svc.post(
            &guide::del_guide_tag_option(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn add_guide_tag_option(
        &self,
        tag_name: &str,
        values: &[String],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"tag_name": tag_name, "tag_values": values});
        svc.post(
            &guide::add_guide_tag_option(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn get_guide_tag_option(&self) -> Result<Vec<WxMpGuideTagInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = svc
            .post(&guide::get_guide_tag_option(config.as_ref()), "{}")
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("tag_option")
            .ok_or_else(|| WxErrorException::from_code(-99, "tag_option 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn add_guide_buyer_tag(
        &self,
        account: &str,
        openid: &str,
        value: &str,
        user_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "tag_value": value, "openid_list": user_open_ids});
        let response = svc
            .post(
                &guide::add_guide_buyer_tag(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::parse_buyer_resp_list(&response)
    }

    async fn get_guide_buyer_tag(
        &self,
        account: &str,
        openid: &str,
        user_openid: &str,
        is_exclude: bool,
    ) -> Result<Vec<String>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": user_openid, "is_exclude": is_exclude});
        let response = svc
            .post(
                &guide::get_guide_buyer_tag(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("tag_values")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .ok_or_else(|| WxErrorException::from_code(-99, "tag_values 缺失"))
    }

    async fn query_guide_buyer_by_tag(
        &self,
        account: &str,
        openid: &str,
        push_count: i32,
        values: &[String],
    ) -> Result<Vec<String>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "push_count": push_count, "tag_values": values});
        let response = svc
            .post(
                &guide::query_guide_buyer_by_tag(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value
            .get("openid_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .ok_or_else(|| WxErrorException::from_code(-99, "openid_list 缺失"))
    }
}
