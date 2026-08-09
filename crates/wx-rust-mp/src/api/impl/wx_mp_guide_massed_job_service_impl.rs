//! WxMpGuideMassedJobService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpGuideMassedJobServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpGuideMassedJobService, WxMpService};
use crate::bean::guide::{
    WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp, WxMpGuideCardMaterialInfo,
    WxMpGuideImgMaterialInfoList, WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo,
    WxMpGuideTagInfo, WxMpGuideWordMaterialInfoList,
};
use crate::enums::wx_mp_api_url::guide;

pub struct WxMpGuideMassedJobServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpGuideMassedJobServiceImpl {
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMpGuideMassedJobService for WxMpGuideMassedJobServiceImpl {
    async fn add_guide_massed_job(
        &self,
        account: &str,
        openid: &str,
        task_name: &str,
        task_remark: &str,
        push_time: i64,
        user_open_ids: &[String],
        material_infos: &[WxMpGuideMaterialInfo],
    ) -> Result<WxMpGuideMassed, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "task_name": task_name, "task_remark": task_remark, "push_time": push_time, "openid_list": user_open_ids, "material_info_list": material_infos});
        let response = svc
            .post(
                &guide::add_guide_massed_job(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_guide_massed_job_list(
        &self,
        account: &str,
        openid: &str,
        task_status: &[i32],
        offset: i32,
        limit: i32,
    ) -> Result<Vec<WxMpGuideMassedInfo>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "task_status": task_status, "offset": offset, "limit": limit});
        let response = svc
            .post(
                &guide::get_guide_massed_job_list(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        let value: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value
            .get("list")
            .ok_or_else(|| WxErrorException::from_code(-99, "list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_guide_massed_job(
        &self,
        task_id: &str,
    ) -> Result<WxMpGuideMassedInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"task_id": task_id});
        let response = svc
            .post(
                &guide::get_guide_massed_job(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        WxMpGuideMassedInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn update_guide_massed_job(
        &self,
        task_id: &str,
        task_name: &str,
        task_remark: &str,
        push_time: i64,
        user_open_ids: &[String],
        material_infos: &[WxMpGuideMaterialInfo],
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"task_id": task_id, "task_name": task_name, "task_remark": task_remark, "push_time": push_time, "openid_list": user_open_ids, "material_info_list": material_infos});
        svc.post(
            &guide::update_guide_massed_job(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }

    async fn cancel_guide_massed_job(&self, task_id: &str) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let body = serde_json::json!({"task_id": task_id});
        svc.post(
            &guide::cancel_guide_massed_job(config.as_ref()),
            &body.to_string(),
        )
        .await?;
        Ok(())
    }
}
