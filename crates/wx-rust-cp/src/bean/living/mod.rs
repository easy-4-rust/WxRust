//! 对应 Java `me.chanjar.weixin.cp.bean.living` 包（生成）。

pub mod wx_cp_living_create_request;
pub mod wx_cp_living_info;
pub mod wx_cp_living_modify_request;
pub mod wx_cp_living_result;
pub mod wx_cp_living_share_info;
pub mod wx_cp_watch_stat;

pub use wx_cp_living_create_request::ActivityDetail;
pub use wx_cp_living_create_request::WxCpLivingCreateRequest;
pub use wx_cp_living_info::WxCpLivingInfo;
pub use wx_cp_living_modify_request::WxCpLivingModifyRequest;
pub use wx_cp_living_result::LivingIdResult;
pub use wx_cp_living_result::WxCpLivingResult;
pub use wx_cp_living_share_info::WxCpLivingShareInfo;
pub use wx_cp_watch_stat::ExternalUser;
pub use wx_cp_watch_stat::StatInfo;
pub use wx_cp_watch_stat::User;
pub use wx_cp_watch_stat::WxCpWatchStat;
