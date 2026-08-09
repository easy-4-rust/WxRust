//! 小程序数据分析相关接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaAnalysisService`。
//! 文档：https://mp.weixin.qq.com/debug/wxadoc/dev/api/analysis.html

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMaRetainInfo, WxMaSummaryTrend, WxMaUserPortrait, WxMaVisitDistribution, WxMaVisitPage,
    WxMaVisitTrend,
};

/// 小程序数据分析相关接口。
#[async_trait]
pub trait WxMaAnalysisService: Send + Sync {
    /// 查询概况趋势（对应 Java `getDailySummaryTrend(Date, Date)`）。
    ///
    /// 温馨提示：小程序接口目前只能查询一天的数据，即 beginDate 和 endDate
    /// 一样。Java 以 `Date` 传参并格式化为 `yyyyMMdd` 字符串
    /// （`DateFormatUtils.format(date, "yyyyMMdd")`）；Rust 直接接收
    /// `yyyyMMdd` 格式日期字符串（ADAPTED）。
    async fn get_daily_summary_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaSummaryTrend>>, WxErrorException>;

    /// 获取日访问趋势（对应 Java `getDailyVisitTrend(Date, Date)`）。
    async fn get_daily_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException>;

    /// 获取周访问趋势（对应 Java `getWeeklyVisitTrend(Date, Date)`）。
    ///
    /// 限定查询一个自然周的数据，时间必须按照自然周的方式输入：
    /// 如：20170306(周一), 20170312(周日)。
    async fn get_weekly_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException>;

    /// 获取月访问趋势（对应 Java `getMonthlyVisitTrend(Date, Date)`）。
    ///
    /// 限定查询一个自然月的数据，时间必须按照自然月的方式输入：
    /// 如：20170201(月初), 20170228(月末)。
    async fn get_monthly_visit_trend(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitTrend>>, WxErrorException>;

    /// 获取访问分布（对应 Java `getVisitDistribution(Date, Date)`）。
    async fn get_visit_distribution(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaVisitDistribution, WxErrorException>;

    /// 获取日留存数据（对应 Java `getDailyRetainInfo(Date, Date)`）。
    async fn get_daily_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException>;

    /// 获取周留存数据（对应 Java `getWeeklyRetainInfo(Date, Date)`）。
    async fn get_weekly_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException>;

    /// 获取月留存数据（对应 Java `getMonthlyRetainInfo(Date, Date)`）。
    async fn get_monthly_retain_info(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaRetainInfo, WxErrorException>;

    /// 获取访问页面数据（对应 Java `getVisitPage(Date, Date)`）。
    async fn get_visit_page(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<Option<Vec<WxMaVisitPage>>, WxErrorException>;

    /// 获取小程序新增或活跃用户的画像分布数据（对应 Java
    /// `getUserPortrait(Date, Date)`）。
    async fn get_user_portrait(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<WxMaUserPortrait, WxErrorException>;
}
