//! 公众号菜单 bean。

pub mod wx_mp_menu;

pub use wx_mp_menu::{WxMpConditionalMenu, WxMpMenu};
pub mod wx_mp_get_self_menu_info_result;
pub mod wx_mp_self_menu_info;
pub use wx_mp_get_self_menu_info_result::WxMpGetSelfMenuInfoResult;
pub use wx_mp_self_menu_info::{WxMpSelfMenuButton, WxMpSelfMenuInfo};
