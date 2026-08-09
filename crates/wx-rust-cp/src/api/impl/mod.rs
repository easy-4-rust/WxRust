//! 企业微信服务实现（对应 Java `me.chanjar.weixin.cp.api.impl` 包）。

pub mod base_wx_cp_service_impl;
pub mod wx_cp_o_mail_service_impl;
pub mod wx_cp_service_impl;
pub mod wx_cp_service_on_tp_impl;

pub use wx_cp_o_mail_service_impl::WxCpOMailServiceImpl;
pub use wx_cp_service_impl::WxCpServiceImpl;
pub use wx_cp_service_on_tp_impl::WxCpServiceOnTpImpl;

// G1 组（核心服务组，Wave 2b I1 独占）：成员/部门/标签/群聊/应用/菜单/
// 消息/素材/OAuth2/群机器人/任务卡片/工作台 12 个子服务实现注册。
pub mod g1_impls;
pub use g1_impls::*;

// G2 组（外部联系人/客服/OA/会话存档/导出/会议/企业互联/智能机器人/
// 人事助手 9 个子服务实现注册，Wave 2b I2 独占文件）。
pub mod g2_impls;
pub use g2_impls::*;

// G3 组（OA/家校/直播 10 个子服务实现注册，Wave 2b I3 独占文件）。
pub mod g3_impls;
pub use g3_impls::*;
