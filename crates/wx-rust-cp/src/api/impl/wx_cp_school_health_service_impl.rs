//! 企业微信家校应用健康上报服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpSchoolHealthServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpSchoolHealthService, WxCpService};
use crate::bean::{
    WxCpGetHealthReportStat, WxCpGetReportAnswer, WxCpGetReportJobIds, WxCpGetReportJobInfo,
};
use crate::enums::url_school;

/// 企业微信家校应用健康上报服务实现。
pub struct WxCpSchoolHealthServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpSchoolHealthServiceImpl {
    /// 构建健康上报服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造获取健康上报统计请求体（对应 Java `getHealthReportStat` 内的
    /// `JsonObject`：`{"date": ...}`）。
    fn build_health_report_stat_body(date: &str) -> String {
        serde_json::json!({ "date": date }).to_string()
    }

    /// 构造获取健康上报任务 ID 列表请求体（对应 Java `getReportJobIds`
    /// 内的 `JsonObject`：`offset` 缺省 0，`limit` 缺省 100）。
    fn build_report_job_ids_body(offset: Option<i32>, limit: Option<i32>) -> String {
        serde_json::json!({
            "offset": offset.unwrap_or(0),
            "limit": limit.unwrap_or(100),
        })
        .to_string()
    }

    /// 构造获取健康上报任务详情请求体（对应 Java `getReportJobInfo` 内的
    /// `JsonObject`：`{"jobid": ..., "date": ...}`）。
    fn build_report_job_info_body(job_id: &str, date: &str) -> String {
        serde_json::json!({
            "jobid": job_id,
            "date": date,
        })
        .to_string()
    }

    /// 构造获取用户填写答案请求体（对应 Java `getReportAnswer` 内的
    /// `JsonObject`：`offset`/`limit` 非空才放入，`jobid`/`date` 必有）。
    fn build_report_answer_body(
        job_id: &str,
        date: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> String {
        let mut body = serde_json::json!({
            "jobid": job_id,
            "date": date,
        });
        if let Some(offset) = offset {
            body["offset"] = serde_json::json!(offset);
        }
        if let Some(limit) = limit {
            body["limit"] = serde_json::json!(limit);
        }
        body.to_string()
    }
}

#[async_trait]
impl WxCpSchoolHealthService for WxCpSchoolHealthServiceImpl {
    async fn get_health_report_stat(
        &self,
        date: &str,
    ) -> Result<WxCpGetHealthReportStat, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getHealthReportStat`：`POST GET_HEALTH_REPORT_STAT`
        //（`date` 最长支持获取 30 天前数据）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_HEALTH_REPORT_STAT);
        let response = svc
            .post(&api_url, &Self::build_health_report_stat_body(date))
            .await?;
        WxCpGetHealthReportStat::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_report_job_ids(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpGetReportJobIds, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getReportJobIds`：`POST GET_REPORT_JOBIDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_REPORT_JOBIDS);
        let response = svc
            .post(&api_url, &Self::build_report_job_ids_body(offset, limit))
            .await?;
        WxCpGetReportJobIds::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_report_job_info(
        &self,
        job_id: &str,
        date: &str,
    ) -> Result<WxCpGetReportJobInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getReportJobInfo`：`POST GET_REPORT_JOB_INFO`（`date` 仅
        // 支持最近 14 天数据）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_REPORT_JOB_INFO);
        let response = svc
            .post(&api_url, &Self::build_report_job_info_body(job_id, date))
            .await?;
        WxCpGetReportJobInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_report_answer(
        &self,
        job_id: &str,
        date: &str,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<WxCpGetReportAnswer, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getReportAnswer`：`POST GET_REPORT_ANSWER`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_REPORT_ANSWER);
        let response = svc
            .post(
                &api_url,
                &Self::build_report_answer_body(job_id, date, offset, limit),
            )
            .await?;
        WxCpGetReportAnswer::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `getReportJobIds`：`offset` 缺省 0，`limit` 缺省 100。
    #[test]
    fn test_build_report_job_ids_body() {
        let body = WxCpSchoolHealthServiceImpl::build_report_job_ids_body(None, None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["offset"], 0);
        assert_eq!(json["limit"], 100);

        let body = WxCpSchoolHealthServiceImpl::build_report_job_ids_body(Some(10), Some(20));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["offset"], 10);
        assert_eq!(json["limit"], 20);
    }

    /// Java `getReportAnswer`：`offset`/`limit` 为空时不放入请求体。
    #[test]
    fn test_build_report_answer_body() {
        let body =
            WxCpSchoolHealthServiceImpl::build_report_answer_body("job1", "2022-06-01", None, None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["jobid"], "job1");
        assert_eq!(json["date"], "2022-06-01");
        assert!(json.get("offset").is_none());
        assert!(json.get("limit").is_none());

        let body = WxCpSchoolHealthServiceImpl::build_report_answer_body(
            "job1",
            "2022-06-01",
            Some(5),
            Some(10),
        );
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["offset"], 5);
        assert_eq!(json["limit"], 10);
    }
}
