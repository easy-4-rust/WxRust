//! 数据分析服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaAnalysisServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use serde::de::DeserializeOwned;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxMaAnalysisService, WxMaService};
use crate::bean::{
    WxMaRetainInfo, WxMaSummaryTrend, WxMaUserPortrait, WxMaVisitDistribution, WxMaVisitPage,
    WxMaVisitTrend,
};
use crate::enums::url_g1_core::analysis as analysis_url;

/// 数据分析服务实现。
pub struct WxMaAnalysisServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaAnalysisServiceImpl {
    /// 构建数据分析服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 日期参数请求体（对应 Java `toJson(Date, Date)`：
    /// `{"begin_date": "yyyyMMdd", "end_date": "yyyyMMdd"}`）。
    fn to_json(begin_date: &str, end_date: &str) -> String {
        serde_json::json!({
            "begin_date": begin_date,
            "end_date": end_date,
        })
        .to_string()
    }

    /// 获取数据分析结果并返回 List（对应 Java `getAnalysisResultAsList`：
    /// 响应含 `list` 字段时解析为列表，否则返回 null → Rust `None`）。
    async fn get_analysis_result_as_list<T: DeserializeOwned>(
        svc: &dyn WxMaService,
        url: &str,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<T>>, WxErrorException> {
        let response = svc.post(url, &Self::to_json(begin_date, end_date)).await?;
        let json: serde_json::Value =
            serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        match json.get("list") {
            Some(list) => serde_json::from_value(list.clone())
                .map(Some)
                .map_err(|e| WxErrorException::Serde(e.to_string())),
            None => Ok(None),
        }
    }

    /// 获取留存数据（对应 Java `getRetainInfo(Date, Date, String)`）。
    async fn get_retain_info(
        svc: &dyn WxMaService,
        url: &str,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException> {
        let response = svc.post(url, &Self::to_json(begin_date, end_date)).await?;
        WxMaRetainInfo::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[async_trait]
impl WxMaAnalysisService for WxMaAnalysisServiceImpl {
    async fn get_daily_summary_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaSummaryTrend>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getDailySummaryTrend`：`GET_DAILY_SUMMARY_TREND_URL`
        let config = svc.wx_ma_config();
        Self::get_analysis_result_as_list(
            svc.as_ref(),
            &analysis_url::get_daily_summary_trend_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_daily_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getDailyVisitTrend`：`GET_DAILY_VISIT_TREND_URL`
        let config = svc.wx_ma_config();
        Self::get_analysis_result_as_list(
            svc.as_ref(),
            &analysis_url::get_daily_visit_trend_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_weekly_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getWeeklyVisitTrend`：`GET_WEEKLY_VISIT_TREND_URL`
        let config = svc.wx_ma_config();
        Self::get_analysis_result_as_list(
            svc.as_ref(),
            &analysis_url::get_weekly_visit_trend_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_monthly_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getMonthlyVisitTrend`：`GET_MONTHLY_VISIT_TREND_URL`
        let config = svc.wx_ma_config();
        Self::get_analysis_result_as_list(
            svc.as_ref(),
            &analysis_url::get_monthly_visit_trend_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_visit_distribution(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaVisitDistribution, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getVisitDistribution`：POST `GET_VISIT_DISTRIBUTION_URL` 后
        // `WxMaVisitDistribution.fromJson`
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &analysis_url::get_visit_distribution_url(config.as_ref()),
                &Self::to_json(begin_date, end_date),
            )
            .await?;
        WxMaVisitDistribution::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_daily_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getDailyRetainInfo`：`GET_DAILY_RETAIN_INFO_URL`
        let config = svc.wx_ma_config();
        Self::get_retain_info(
            svc.as_ref(),
            &analysis_url::get_daily_retain_info_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_weekly_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getWeeklyRetainInfo`：`GET_WEEKLY_RETAIN_INFO_URL`
        let config = svc.wx_ma_config();
        Self::get_retain_info(
            svc.as_ref(),
            &analysis_url::get_weekly_retain_info_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_monthly_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getMonthlyRetainInfo`：`GET_MONTHLY_RETAIN_INFO_URL`
        let config = svc.wx_ma_config();
        Self::get_retain_info(
            svc.as_ref(),
            &analysis_url::get_monthly_retain_info_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_visit_page(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitPage>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getVisitPage`：`GET_VISIT_PAGE_URL`
        let config = svc.wx_ma_config();
        Self::get_analysis_result_as_list(
            svc.as_ref(),
            &analysis_url::get_visit_page_url(config.as_ref()),
            begin_date,
            end_date,
        )
        .await
    }

    async fn get_user_portrait(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaUserPortrait, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        // Java `getUserPortrait`：POST `GET_USER_PORTRAIT_URL` 后
        // `WxMaUserPortrait.fromJson`
        let config = svc.wx_ma_config();
        let response = svc
            .post(
                &analysis_url::get_user_portrait_url(config.as_ref()),
                &Self::to_json(begin_date, end_date),
            )
            .await?;
        WxMaUserPortrait::from_json(&response).map_err(WxErrorException::Serde)
    }
}
