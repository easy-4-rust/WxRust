//! 接口返回结果 bean。

pub mod wx_mp_change_openid;
pub mod wx_mp_current_auto_reply_info;
pub mod wx_mp_mass_get_result;
pub mod wx_mp_mass_send_result;
pub mod wx_mp_mass_speed_get_result;
pub mod wx_mp_mass_upload_result;
pub mod wx_mp_qr_code_ticket;
pub mod wx_mp_semantic_query_result;
pub mod wx_mp_short_key_result;
pub mod wx_mp_user;
pub mod wx_mp_user_blacklist_get_result;
pub mod wx_mp_user_list;

pub use wx_mp_change_openid::WxMpChangeOpenid;
pub use wx_mp_current_auto_reply_info::{
    AutoReplyInfo, AutoReplyRule, KeywordAutoReplyInfo, KeywordInfo, NewsInfo, NewsItem, ReplyInfo,
    WxMpCurrentAutoReplyInfo,
};
pub use wx_mp_mass_get_result::WxMpMassGetResult;
pub use wx_mp_mass_send_result::WxMpMassSendResult;
pub use wx_mp_mass_speed_get_result::WxMpMassSpeedGetResult;
pub use wx_mp_mass_upload_result::WxMpMassUploadResult;
pub use wx_mp_qr_code_ticket::WxMpQrCodeTicket;
pub use wx_mp_semantic_query_result::WxMpSemanticQueryResult;
pub use wx_mp_short_key_result::WxMpShortKeyResult;
pub use wx_mp_user::WxMpUser;
pub use wx_mp_user_blacklist_get_result::WxMpUserBlacklistGetResult;
pub use wx_mp_user_list::WxMpUserList;
