//! G1 组（核心服务组）子服务实现注册。
//!
//! 本组实现 Java `me.chanjar.weixin.cp.api.impl` 中 13 个核心子服务：
//! WxCpUserServiceImpl/WxCpDepartmentServiceImpl/WxCpTagServiceImpl/
//! WxCpChatServiceImpl/WxCpAgentServiceImpl/WxCpMenuServiceImpl/
//! WxCpMessageServiceImpl/WxCpMediaServiceImpl/WxCpOAuth2ServiceImpl/
//! WxCpGroupRobotServiceImpl/WxCpTaskCardServiceImpl/
//! WxCpAgentWorkBenchServiceImpl。
//! 由后续批次统一装配到门面实现（对应 Java Base 构造器中的子服务字段）。
//!
//! 模块文件位于 `api/impl/` 根目录（`wx_cp_<域>_service_impl.rs`，与任务
//! 文件布局一致）；本文件为非 `mod.rs` 的分组注册文件，子模块以 `#[path]`
//! 显式指回根目录文件。

#[path = "wx_cp_agent_service_impl.rs"]
pub mod wx_cp_agent_service_impl;
#[path = "wx_cp_agent_work_bench_service_impl.rs"]
pub mod wx_cp_agent_work_bench_service_impl;
#[path = "wx_cp_chat_service_impl.rs"]
pub mod wx_cp_chat_service_impl;
#[path = "wx_cp_department_service_impl.rs"]
pub mod wx_cp_department_service_impl;
#[path = "wx_cp_group_robot_service_impl.rs"]
pub mod wx_cp_group_robot_service_impl;
#[path = "wx_cp_media_service_impl.rs"]
pub mod wx_cp_media_service_impl;
#[path = "wx_cp_menu_service_impl.rs"]
pub mod wx_cp_menu_service_impl;
#[path = "wx_cp_message_service_impl.rs"]
pub mod wx_cp_message_service_impl;
#[path = "wx_cp_oauth2_service_impl.rs"]
pub mod wx_cp_oauth2_service_impl;
#[path = "wx_cp_tag_service_impl.rs"]
pub mod wx_cp_tag_service_impl;
#[path = "wx_cp_task_card_service_impl.rs"]
pub mod wx_cp_task_card_service_impl;
#[path = "wx_cp_user_service_impl.rs"]
pub mod wx_cp_user_service_impl;

pub use wx_cp_agent_service_impl::WxCpAgentServiceImpl;
pub use wx_cp_agent_work_bench_service_impl::WxCpAgentWorkBenchServiceImpl;
pub use wx_cp_chat_service_impl::WxCpChatServiceImpl;
pub use wx_cp_department_service_impl::WxCpDepartmentServiceImpl;
pub use wx_cp_group_robot_service_impl::WxCpGroupRobotServiceImpl;
pub use wx_cp_media_service_impl::WxCpMediaServiceImpl;
pub use wx_cp_menu_service_impl::WxCpMenuServiceImpl;
pub use wx_cp_message_service_impl::WxCpMessageServiceImpl;
pub use wx_cp_oauth2_service_impl::WxCpOAuth2ServiceImpl;
pub use wx_cp_tag_service_impl::WxCpTagServiceImpl;
pub use wx_cp_task_card_service_impl::WxCpTaskCardServiceImpl;
pub use wx_cp_user_service_impl::WxCpUserServiceImpl;
