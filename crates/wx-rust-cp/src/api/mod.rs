//! 企业微信 API（门面与子服务）。
//!
//! 对应 Java `me.chanjar.weixin.cp.api` 包。
//!
//! 31 个子服务 trait 各占一个文件（`wx_cp_<域>_service.rs`，文件命名
//! 对照 miniapp 的 `wx_ma_<域>_service.rs`），trait 名与 Java
//! `WxCpXxxService` 一一对应，方法签名镜像 Java 原语义（见各文件内
//! 中文注释标注的 Java 接口名与方法）。

pub mod r#impl;
pub mod wx_cp_agent_service;
pub mod wx_cp_agent_work_bench_service;
pub mod wx_cp_chat_service;
pub mod wx_cp_corp_group_service;
pub mod wx_cp_department_service;
pub mod wx_cp_export_service;
pub mod wx_cp_external_contact_service;
pub mod wx_cp_group_robot_service;
pub mod wx_cp_hr_service;
pub mod wx_cp_intelligent_robot_service;
pub mod wx_cp_kf_service;
pub mod wx_cp_living_service;
pub mod wx_cp_media_service;
pub mod wx_cp_meeting_service;
pub mod wx_cp_menu_service;
pub mod wx_cp_message_service;
pub mod wx_cp_msg_audit_service;
pub mod wx_cp_oa_agent_service;
pub mod wx_cp_oa_calendar_service;
pub mod wx_cp_oa_mail_service;
pub mod wx_cp_oa_meeting_room_service;
pub mod wx_cp_oa_schedule_service;
pub mod wx_cp_oa_service;
pub mod wx_cp_oa_we_doc_service;
pub mod wx_cp_oa_we_drive_service;
pub mod wx_cp_oauth2_service;
pub mod wx_cp_school_health_service;
pub mod wx_cp_school_service;
pub mod wx_cp_school_user_service;
pub mod wx_cp_service;
pub mod wx_cp_tag_service;
pub mod wx_cp_task_card_service;
pub mod wx_cp_todo_service;
pub mod wx_cp_user_service;

pub use wx_cp_agent_service::WxCpAgentService;
pub use wx_cp_agent_work_bench_service::WxCpAgentWorkBenchService;
pub use wx_cp_chat_service::WxCpChatService;
pub use wx_cp_corp_group_service::WxCpCorpGroupService;
pub use wx_cp_department_service::WxCpDepartmentService;
pub use wx_cp_export_service::WxCpExportService;
pub use wx_cp_external_contact_service::WxCpExternalContactService;
pub use wx_cp_group_robot_service::WxCpGroupRobotService;
pub use wx_cp_hr_service::WxCpHrService;
pub use wx_cp_intelligent_robot_service::WxCpIntelligentRobotService;
pub use wx_cp_kf_service::WxCpKfService;
pub use wx_cp_living_service::WxCpLivingService;
pub use wx_cp_media_service::WxCpMediaService;
pub use wx_cp_meeting_service::WxCpMeetingService;
pub use wx_cp_menu_service::WxCpMenuService;
pub use wx_cp_message_service::WxCpMessageService;
pub use wx_cp_msg_audit_service::WxCpMsgAuditService;
pub use wx_cp_oa_agent_service::WxCpOaAgentService;
pub use wx_cp_oa_calendar_service::WxCpOaCalendarService;
pub use wx_cp_oa_mail_service::WxCpOaMailService;
pub use wx_cp_oa_meeting_room_service::WxCpOaMeetingRoomService;
pub use wx_cp_oa_schedule_service::WxCpOaScheduleService;
pub use wx_cp_oa_service::WxCpOaService;
pub use wx_cp_oa_we_doc_service::WxCpOaWeDocService;
pub use wx_cp_oa_we_drive_service::WxCpOaWeDriveService;
pub use wx_cp_oauth2_service::WxCpOAuth2Service;
pub use wx_cp_school_health_service::WxCpSchoolHealthService;
pub use wx_cp_school_service::WxCpSchoolService;
pub use wx_cp_school_user_service::WxCpSchoolUserService;
pub use wx_cp_service::WxCpService;
pub use wx_cp_tag_service::WxCpTagService;
pub use wx_cp_task_card_service::WxCpTaskCardService;
pub use wx_cp_todo_service::WxCpTodoService;
pub use wx_cp_user_service::WxCpUserService;
