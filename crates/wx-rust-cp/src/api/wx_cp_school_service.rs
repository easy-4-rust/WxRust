//! 企业微信家校应用复学码服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpSchoolService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    LivingIdResult, WxCpCustomizeHealthInfo, WxCpLivingResult, WxCpPaymentResult, WxCpResultList,
    WxCpSchoolLivingInfo, WxCpSchoolUnwatchStat, WxCpSchoolWatchStat, WxCpTrade,
};

/// 企业微信家校应用复学码服务。
#[async_trait]
pub trait WxCpSchoolService: Send + Sync {
    /// 获取老师健康信息（对应 Java
    /// `WxCpSchoolService.getTeacherCustomizeHealthInfo(String, String,
    /// Integer)`）。
    async fn get_teacher_customize_health_info(
        &self,
        date: &str,
        next_key: &str,
        limit: Option<i32>,
    ) -> Result<WxCpCustomizeHealthInfo, WxErrorException>;

    /// 获取学生健康信息（对应 Java
    /// `WxCpSchoolService.getStudentCustomizeHealthInfo(String, String,
    /// Integer)`）。
    async fn get_student_customize_health_info(
        &self,
        date: &str,
        next_key: &str,
        limit: Option<i32>,
    ) -> Result<WxCpCustomizeHealthInfo, WxErrorException>;

    /// 获取师生健康码（对应 Java
    /// `WxCpSchoolService.getHealthQrCode(List<String>, Integer)`）。
    async fn get_health_qr_code(
        &self,
        user_ids: &[&str],
        r#type: Option<i32>,
    ) -> Result<WxCpResultList, WxErrorException>;

    /// 获取学生付款结果（对应 Java
    /// `WxCpSchoolService.getPaymentResult(String)`）。
    async fn get_payment_result(
        &self,
        payment_id: &str,
    ) -> Result<WxCpPaymentResult, WxErrorException>;

    /// 获取订单详情（对应 Java
    /// `WxCpSchoolService.getTrade(String, String)`）。
    async fn get_trade(
        &self,
        payment_id: &str,
        trade_no: &str,
    ) -> Result<WxCpTrade, WxErrorException>;

    /// 获取直播详情（对应 Java
    /// `WxCpSchoolService.getLivingInfo(String)`）。
    async fn get_living_info(
        &self,
        living_id: &str,
    ) -> Result<WxCpSchoolLivingInfo, WxErrorException>;

    /// 获取老师直播 ID 列表（对应 Java
    /// `WxCpSchoolService.getUserAllLivingId(String, String, Integer)`）。
    async fn get_user_all_living_id(
        &self,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<LivingIdResult, WxErrorException>;

    /// 获取观看直播统计（对应 Java
    /// `WxCpSchoolService.getWatchStat(String, String)`）。
    async fn get_watch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpSchoolWatchStat, WxErrorException>;

    /// 获取未观看直播统计（对应 Java
    /// `WxCpSchoolService.getUnwatchStat(String, String)`）。
    async fn get_unwatch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpSchoolUnwatchStat, WxErrorException>;

    /// 删除直播回放（对应 Java
    /// `WxCpSchoolService.deleteReplayData(String)`）。
    async fn delete_replay_data(
        &self,
        living_id: &str,
    ) -> Result<WxCpLivingResult, WxErrorException>;
}
