//! 企业微信第三方应用 ID 转换服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service.impl.WxCpTpIdConvertServiceImpl`：
//! 以 `Weak<dyn WxCpTpService>` 持有门面。ID 转换接口使用授权企业的
//! access_token（`config.getAccessToken(corpId)`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpTpConvertTmpExternalUserIdResult, WxCpTpOpenKfIdConvertResult,
    WxCpTpTagIdListConvertResult, WxCpTpUnionidToExternalUseridResult,
};
use crate::enums::url_id_convert;
use crate::tp::service::{WxCpTpIdConvertService, WxCpTpService};

/// 企业微信第三方应用 ID 转换服务实现。
pub struct WxCpTpIdConvertServiceImpl {
    service: Weak<dyn WxCpTpService>,
}

impl WxCpTpIdConvertServiceImpl {
    /// 构建服务（对应 Java 构造器注入 `WxCpTpService`）。
    pub fn new(service: Weak<dyn WxCpTpService>) -> Self {
        Self { service }
    }

    fn service(&self) -> Result<Arc<dyn WxCpTpService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "WxCpTpService 引用已失效"))
    }

    /// 拼接带授权企业 access_token 的 URL（对应 Java 各方法内
    /// `getApiUrl(path) + "?access_token=" + getAccessToken(corpId)`）。
    fn url_with_corp_token(
        &self,
        service: &dyn WxCpTpService,
        corp_id: &str,
        path: &str,
    ) -> String {
        let config = service.wx_cp_tp_config_storage();
        format!(
            "{}?access_token={}",
            config.api_url(path),
            config.access_token(corp_id).unwrap_or_default()
        )
    }
}

#[async_trait]
impl WxCpTpIdConvertService for WxCpTpIdConvertServiceImpl {
    async fn unionid_to_external_userid(
        &self,
        corp_id: &str,
        unionid: &str,
        openid: &str,
        subject_type: Option<i32>,
    ) -> Result<WxCpTpUnionidToExternalUseridResult, WxErrorException> {
        let service = self.service()?;
        let mut body = serde_json::Map::new();
        body.insert(
            "unionid".to_string(),
            serde_json::Value::String(unionid.to_string()),
        );
        body.insert(
            "openid".to_string(),
            serde_json::Value::String(openid.to_string()),
        );
        if let Some(subject_type) = subject_type {
            body.insert(
                "subject_type".to_string(),
                serde_json::Value::Number(subject_type.into()),
            );
        }
        let url = self.url_with_corp_token(
            service.as_ref(),
            corp_id,
            url_id_convert::UNION_ID_TO_EXTERNAL_USER_ID,
        );
        let response_content = service
            .post(&url, &serde_json::Value::Object(body).to_string())
            .await?;
        WxCpTpUnionidToExternalUseridResult::from_json(&response_content)
            .map_err(WxErrorException::Serde)
    }

    async fn external_tag_id(
        &self,
        corp_id: &str,
        external_tag_id_list: &[String],
    ) -> Result<WxCpTpTagIdListConvertResult, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "external_tagid_list": external_tag_id_list });
        let url =
            self.url_with_corp_token(service.as_ref(), corp_id, url_id_convert::EXTERNAL_TAG_ID);
        let response_content = service.post(&url, &body.to_string()).await?;
        WxCpTpTagIdListConvertResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn convert_open_kf_id(
        &self,
        corp_id: &str,
        open_kf_id_list: &[String],
    ) -> Result<WxCpTpOpenKfIdConvertResult, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({ "open_kfid_list": open_kf_id_list });
        let url = self.url_with_corp_token(service.as_ref(), corp_id, url_id_convert::OPEN_KF_ID);
        let response_content = service.post(&url, &body.to_string()).await?;
        WxCpTpOpenKfIdConvertResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    async fn convert_tmp_external_user_id(
        &self,
        corp_id: &str,
        business_type: i32,
        user_type: i32,
        tmp_external_user_id_list: &[String],
    ) -> Result<WxCpTpConvertTmpExternalUserIdResult, WxErrorException> {
        let service = self.service()?;
        let body = serde_json::json!({
            "business_type": business_type,
            "user_type": user_type,
            "tmp_external_userid_list": tmp_external_user_id_list,
        });
        let url = self.url_with_corp_token(
            service.as_ref(),
            corp_id,
            url_id_convert::CONVERT_TMP_EXTERNAL_USER_ID,
        );
        let response_content = service.post(&url, &body.to_string()).await?;
        WxCpTpConvertTmpExternalUserIdResult::from_json(&response_content)
            .map_err(WxErrorException::Serde)
    }
}
