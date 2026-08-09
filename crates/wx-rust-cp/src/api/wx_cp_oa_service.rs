//! 企业微信 OA 服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaService`。

use async_trait::async_trait;

use chrono::{DateTime, Utc};
use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpApprovalDetailResult, WxCpApprovalInfo, WxCpApprovalInfoQueryFilter, WxCpBaseResp,
    WxCpCheckinData, WxCpCheckinDayData, WxCpCheckinMonthData, WxCpCheckinOption,
    WxCpCheckinSchedule, WxCpCorpConfInfo, WxCpCropCheckinOption, WxCpDialRecord,
    WxCpGetApprovalData, WxCpOaApplyEventRequest, WxCpOaApprovalTemplate,
    WxCpOaApprovalTemplateResult, WxCpSetCheckinSchedule, WxCpUserVacationQuota,
};

/// 企业微信 OA 服务。
#[async_trait]
pub trait WxCpOaService: Send + Sync {
    /// 提交审批申请（对应 Java
    /// `WxCpOaService.apply(WxCpOaApplyEventRequest)`，返回表单编号）。
    async fn apply(&self, request: &WxCpOaApplyEventRequest) -> Result<String, WxErrorException>;

    /// 获取打卡数据（对应 Java
    /// `WxCpOaService.getCheckinData(Integer, Date, Date, List<String>)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED；
    /// `openCheckinDataType`：1-上下班打卡，2-外出打卡，3-全部打卡）。
    async fn get_checkin_data(
        &self,
        open_checkin_data_type: i32,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinData>, WxErrorException>;

    /// 获取打卡规则（对应 Java
    /// `WxCpOaService.getCheckinOption(Date, List<String>)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_checkin_option(
        &self,
        datetime: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinOption>, WxErrorException>;

    /// 获取企业所有打卡规则（对应 Java
    /// `WxCpOaService.getCropCheckinOption()`）。
    async fn get_crop_checkin_option(&self)
    -> Result<Vec<WxCpCropCheckinOption>, WxErrorException>;

    /// 批量获取审批单号（旧分页游标版，对应 Java
    /// `WxCpOaService.getApprovalInfo(Date, Date, Integer, Integer,
    /// List<WxCpApprovalInfoQueryFilter>)`，Java 中已 `@Deprecated`，
    /// 推荐使用 `get_approval_info_with_new_cursor`）。
    async fn get_approval_info_with_cursor(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        cursor: Option<i32>,
        size: Option<i32>,
        filters: Option<&[WxCpApprovalInfoQueryFilter]>,
    ) -> Result<WxCpApprovalInfo, WxErrorException>;

    /// 批量获取审批单号（简版，对应 Java
    /// `WxCpOaService.getApprovalInfo(Date, Date)`，Java 中已
    /// `@Deprecated`，推荐使用 `get_approval_info_with_new_cursor`）。
    async fn get_approval_info(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<WxCpApprovalInfo, WxErrorException>;

    /// 批量获取审批单号（新分页游标版，对应 Java
    /// `WxCpOaService.getApprovalInfo(Date, Date, String, Integer,
    /// List<WxCpApprovalInfoQueryFilter>)`）。
    async fn get_approval_info_with_new_cursor(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        new_cursor: Option<&str>,
        size: Option<i32>,
        filters: Option<&[WxCpApprovalInfoQueryFilter]>,
    ) -> Result<WxCpApprovalInfo, WxErrorException>;

    /// 获取审批申请详情（对应 Java
    /// `WxCpOaService.getApprovalDetail(String)`）。
    async fn get_approval_detail(
        &self,
        sp_no: &str,
    ) -> Result<WxCpApprovalDetailResult, WxErrorException>;

    /// 获取企业假期管理配置（对应 Java `WxCpOaService.getCorpConf()`）。
    async fn get_corp_conf(&self) -> Result<WxCpCorpConfInfo, WxErrorException>;

    /// 获取成员假期余额（对应 Java
    /// `WxCpOaService.getUserVacationQuota(String)`）。
    async fn get_user_vacation_quota(
        &self,
        user_id: &str,
    ) -> Result<WxCpUserVacationQuota, WxErrorException>;

    /// 获取审批数据（旧，对应 Java
    /// `WxCpOaService.getApprovalData(Long, Long, Long)`；参数为 Unix
    /// 时间戳）。
    async fn get_approval_data(
        &self,
        start_time: i64,
        end_time: i64,
        next_sp_num: Option<i64>,
    ) -> Result<WxCpGetApprovalData, WxErrorException>;

    /// 修改成员假期余额（对应 Java
    /// `WxCpOaService.setOneUserQuota(String, Integer, Integer, Integer,
    /// String)`）。
    async fn set_one_user_quota(
        &self,
        user_id: &str,
        vacation_id: i32,
        left_duration: i32,
        time_attr: i32,
        remarks: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取公费电话拨打记录（对应 Java
    /// `WxCpOaService.getDialRecord(Date, Date, Integer, Integer)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_dial_record(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> Result<Vec<WxCpDialRecord>, WxErrorException>;

    /// 获取审批模板详情（对应 Java
    /// `WxCpOaService.getTemplateDetail(String)`）。
    async fn get_template_detail(
        &self,
        template_id: &str,
    ) -> Result<WxCpOaApprovalTemplateResult, WxErrorException>;

    /// 创建审批模板（对应 Java
    /// `WxCpOaService.createOaApprovalTemplate(WxCpOaApprovalTemplate)`，
    /// 返回 templateId）。
    async fn create_oa_approval_template(
        &self,
        cp_template: &WxCpOaApprovalTemplate,
    ) -> Result<String, WxErrorException>;

    /// 更新审批模板（对应 Java
    /// `WxCpOaService.updateOaApprovalTemplate(WxCpOaApprovalTemplate)`）。
    async fn update_oa_approval_template(
        &self,
        wx_cp_template: &WxCpOaApprovalTemplate,
    ) -> Result<(), WxErrorException>;

    /// 获取打卡日报数据（对应 Java
    /// `WxCpOaService.getCheckinDayData(Date, Date, List<String>)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_checkin_day_data(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinDayData>, WxErrorException>;

    /// 获取打卡月报数据（对应 Java
    /// `WxCpOaService.getCheckinMonthData(Date, Date, List<String>)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_checkin_month_data(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinMonthData>, WxErrorException>;

    /// 获取打卡人员排班信息（对应 Java
    /// `WxCpOaService.getCheckinScheduleList(Date, Date, List<String>)`；
    /// Java `Date` 以 `chrono::DateTime<Utc>` 表达，ADAPTED）。
    async fn get_checkin_schedule_list(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        user_id_list: &[&str],
    ) -> Result<Vec<WxCpCheckinSchedule>, WxErrorException>;

    /// 为打卡人员排班（对应 Java
    /// `WxCpOaService.setCheckinScheduleList(WxCpSetCheckinSchedule)`）。
    async fn set_checkin_schedule_list(
        &self,
        wx_cp_set_checkin_schedule: &WxCpSetCheckinSchedule,
    ) -> Result<(), WxErrorException>;

    /// 录入打卡人员人脸信息（对应 Java
    /// `WxCpOaService.addCheckInUserFace(String, String)`；`userFace`
    /// 为 base64 编码的图片数据）。
    async fn add_check_in_user_face(
        &self,
        user_id: &str,
        user_face: &str,
    ) -> Result<(), WxErrorException>;
}
