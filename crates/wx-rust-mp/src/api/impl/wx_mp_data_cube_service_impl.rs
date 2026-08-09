//! WxMpDataCubeService 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.WxMpDataCubeServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMpDataCubeService, WxMpService};

use crate::bean::datacube::{
    WxDataCubeArticleResult, WxDataCubeArticleTotal, WxDataCubeUserCumulate, WxDataCubeUserSummary,
};
use crate::enums::wx_mp_api_url::datacube;

/// WxMpDataCube服务实现。
pub struct WxMpDataCubeServiceImpl {
    service: Weak<dyn WxMpService>,
}

impl WxMpDataCubeServiceImpl {
    /// 构建 WxMpDataCube服务。
    pub fn new(service: Weak<dyn WxMpService>) -> Self {
        Self { service }
    }

    /// 带起止日期参数的统计查询（对应 Java `buildParams`：begin_date/end_date）。
    async fn post_dates(
        svc: &dyn WxMpService,
        url: &str,
        begin_date: &str,
        end_date: &str,
    ) -> Result<String, WxErrorException> {
        let body = serde_json::json!({"begin_date": begin_date, "end_date": end_date});
        svc.post(url, &body.to_string()).await
    }
}

#[async_trait]
impl WxMpDataCubeService for WxMpDataCubeServiceImpl {
    async fn get_user_summary(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeUserSummary>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_dates(
            svc.as_ref(),
            &datacube::get_user_summary(config.as_ref()),
            begin_date,
            end_date,
        )
        .await?;
        WxDataCubeUserSummary::from_json_list(&response).map_err(WxErrorException::Serde)
    }

    async fn get_user_cumulate(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeUserCumulate>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_dates(
            svc.as_ref(),
            &datacube::get_user_cumulate(config.as_ref()),
            begin_date,
            end_date,
        )
        .await?;
        WxDataCubeUserCumulate::from_json_list(&response).map_err(WxErrorException::Serde)
    }

    async fn get_article_summary(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeArticleResult>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_dates(
            svc.as_ref(),
            &datacube::get_article_summary(config.as_ref()),
            begin_date,
            end_date,
        )
        .await?;
        WxDataCubeArticleResult::from_json_list(&response).map_err(WxErrorException::Serde)
    }

    async fn get_article_total(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeArticleTotal>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;
        let config = svc.wx_mp_config_storage();
        let response = Self::post_dates(
            svc.as_ref(),
            &datacube::get_article_total(config.as_ref()),
            begin_date,
            end_date,
        )
        .await?;
        WxDataCubeArticleTotal::from_json_list(&response).map_err(WxErrorException::Serde)
    }
}
