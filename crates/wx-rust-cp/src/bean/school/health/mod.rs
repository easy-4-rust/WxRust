//! 对应 Java `me.chanjar.weixin.cp.bean.school/health` 包（生成）。

pub mod wx_cp_get_health_report_stat;
pub mod wx_cp_get_report_answer;
pub mod wx_cp_get_report_job_ids;
pub mod wx_cp_get_report_job_info;

pub use wx_cp_get_health_report_stat::WxCpGetHealthReportStat;
pub use wx_cp_get_report_answer::Answer;
pub use wx_cp_get_report_answer::ReportValue;
pub use wx_cp_get_report_answer::WxCpGetReportAnswer;
pub use wx_cp_get_report_job_ids::WxCpGetReportJobIds;
pub use wx_cp_get_report_job_info::ApplyRange;
pub use wx_cp_get_report_job_info::JobInfo;
pub use wx_cp_get_report_job_info::OptionList;
pub use wx_cp_get_report_job_info::QuestionTemplate;
pub use wx_cp_get_report_job_info::ReportTo;
pub use wx_cp_get_report_job_info::WxCpGetReportJobInfo;
