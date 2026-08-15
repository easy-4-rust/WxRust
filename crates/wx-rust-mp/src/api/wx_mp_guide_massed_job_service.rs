//! WxMpGuideMassedJobService。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.WxMpGuideMassedJobService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::guide::{WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo};

/// 公众号GuideMassedJobService。
#[async_trait]
pub trait WxMpGuideMassedJobService: Send + Sync {
    async fn add_guide_massed_job(
        &self,
        account: &str,
        openid: &str,
        task_name: &str,
        task_remark: &str,
        push_time: i64,
        user_open_ids: &[String],
        material_infos: &[WxMpGuideMaterialInfo],
    ) -> Result<WxMpGuideMassed, WxErrorException>;

    async fn get_guide_massed_job_list(
        &self,
        account: &str,
        openid: &str,
        task_status: &[i32],
        offset: i32,
        limit: i32,
    ) -> Result<Vec<WxMpGuideMassedInfo>, WxErrorException>;

    async fn get_guide_massed_job(
        &self,
        task_id: &str,
    ) -> Result<WxMpGuideMassedInfo, WxErrorException>;

    async fn update_guide_massed_job(
        &self,
        task_id: &str,
        task_name: &str,
        task_remark: &str,
        push_time: i64,
        user_open_ids: &[String],
        material_infos: &[WxMpGuideMaterialInfo],
    ) -> Result<(), WxErrorException>;

    async fn cancel_guide_massed_job(&self, task_id: &str) -> Result<(), WxErrorException>;
}
