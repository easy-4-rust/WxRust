//! WxMpDataCube服务
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpDataCubeService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::datacube::{
    WxDataCubeArticleResult, WxDataCubeArticleTotal, WxDataCubeUserCumulate, WxDataCubeUserSummary,
};

/// WxMpDataCube服务。
#[async_trait]
pub trait WxMpDataCubeService: Send + Sync {
    async fn get_user_summary(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeUserSummary>, WxErrorException>;

    async fn get_user_cumulate(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeUserCumulate>, WxErrorException>;

    async fn get_article_summary(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeArticleResult>, WxErrorException>;

    async fn get_article_total(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Vec<WxDataCubeArticleTotal>, WxErrorException>;
}
