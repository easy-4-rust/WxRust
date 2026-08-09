//! 企业微信配置存储默认实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.config.impl` 包（内存实现；
//! redis/redisson 等宿主实现以 `PLATFORM_NA` 归类）。

pub mod wx_cp_corp_group_default_config_impl;
pub mod wx_cp_default_config_impl;
pub mod wx_cp_tp_default_config_impl;

pub use wx_cp_corp_group_default_config_impl::WxCpCorpGroupDefaultConfig;
pub use wx_cp_default_config_impl::WxCpDefaultConfig;
pub use wx_cp_tp_default_config_impl::WxCpTpDefaultConfig;
