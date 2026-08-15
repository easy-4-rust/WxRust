//! WxMpGuideBuyerService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpGuideBuyerServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpGuideBuyerService, WxMpService};
use crate::bean::guide::{WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp};
use crate::enums::wx_mp_api_url::guide;

pub struct WxMpGuideBuyerServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpGuideBuyerServiceImpl {
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 解析买家操作响应列表（对应 Java `GsonParser.parse(json).get("buyer_resp").getAsJsonArray()`）。
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
impl WxMpGuideBuyerService for WxMpGuideBuyerServiceImpl {
    async fn add_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        infos: &[WxMpAddGuideBuyerInfo],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "buyer_list": infos});
        let response = svc
            .post(
                &guide::add_guide_buyer_relation(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::parse_buyer_resp_list(&response)
    }

    async fn del_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        buyer_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid_list": buyer_open_ids});
        let response = svc
            .post(
                &guide::del_guide_buyer_relation(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::parse_buyer_resp_list(&response)
    }

    async fn get_guide_buyer_relation_list(
        &self,
        account: &str,
        openid: &str,
        page: i32,
        num: i32,
    ) -> Result<WxMpGuideBuyerInfoList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "page": page, "num": num});
        let response = svc
            .post(
                &guide::get_guide_buyer_relation_list(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("buyer_list")
            .ok_or_else(|| WxErrorException::from_code(-99, "buyer_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn rebind_guide_acct_for_buyer(
        &self,
        old_account: &str,
        old_openid: &str,
        account: &str,
        openid: &str,
        buyer_open_ids: &[String],
    ) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"old_guide_account": old_account, "old_guide_openid": old_openid, "guide_account": account, "guide_openid": openid, "openid_list": buyer_open_ids});
        let response = svc
            .post(
                &guide::rebind_guide_acct_for_buyer(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        Self::parse_buyer_resp_list(&response)
    }

    async fn update_guide_buyer_relation(
        &self,
        account: &str,
        openid: &str,
        user_openid: &str,
        nickname: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": user_openid, "nickname": nickname});
        svc.post(
            &guide::update_guide_buyer_relation(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }
}
