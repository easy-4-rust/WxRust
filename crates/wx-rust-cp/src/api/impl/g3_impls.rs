//! OA/家校/直播服务组（G3）子服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl` 包中 OA（自建应用/日历/
//! 会议室/日程/微文档/微盘）、家校（复学码/健康上报/家校沟通）、直播
//! 类子服务实现（10 个）。
//!
//! 模块文件位于 `api/impl/` 根目录（`wx_cp_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。
//!
//! 注册方式：由 `api/impl/mod.rs`（I1 独占）追加以下两行完成装配：
//! ```text
//! pub mod g3_impls;
//! pub use g3_impls::*;
//! ```

#[path = "wx_cp_living_service_impl.rs"]
pub mod wx_cp_living_service_impl;
#[path = "wx_cp_oa_agent_service_impl.rs"]
pub mod wx_cp_oa_agent_service_impl;
#[path = "wx_cp_oa_calendar_service_impl.rs"]
pub mod wx_cp_oa_calendar_service_impl;
#[path = "wx_cp_oa_meeting_room_service_impl.rs"]
pub mod wx_cp_oa_meeting_room_service_impl;
#[path = "wx_cp_oa_schedule_service_impl.rs"]
pub mod wx_cp_oa_schedule_service_impl;
#[path = "wx_cp_oa_we_doc_service_impl.rs"]
pub mod wx_cp_oa_we_doc_service_impl;
#[path = "wx_cp_oa_we_drive_service_impl.rs"]
pub mod wx_cp_oa_we_drive_service_impl;
#[path = "wx_cp_school_health_service_impl.rs"]
pub mod wx_cp_school_health_service_impl;
#[path = "wx_cp_school_service_impl.rs"]
pub mod wx_cp_school_service_impl;
#[path = "wx_cp_school_user_service_impl.rs"]
pub mod wx_cp_school_user_service_impl;

pub use wx_cp_living_service_impl::WxCpLivingServiceImpl;
pub use wx_cp_oa_agent_service_impl::WxCpOaAgentServiceImpl;
pub use wx_cp_oa_calendar_service_impl::WxCpOaCalendarServiceImpl;
pub use wx_cp_oa_meeting_room_service_impl::WxCpOaMeetingRoomServiceImpl;
pub use wx_cp_oa_schedule_service_impl::WxCpOaScheduleServiceImpl;
pub use wx_cp_oa_we_doc_service_impl::WxCpOaWeDocServiceImpl;
pub use wx_cp_oa_we_drive_service_impl::WxCpOaWeDriveServiceImpl;
pub use wx_cp_school_health_service_impl::WxCpSchoolHealthServiceImpl;
pub use wx_cp_school_service_impl::WxCpSchoolServiceImpl;
pub use wx_cp_school_user_service_impl::WxCpSchoolUserServiceImpl;
