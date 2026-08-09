//! 对应 Java `me.chanjar.weixin.cp.bean.oa/mail` 包（生成）。

pub mod wx_cp_mail_common_send_request;
pub mod wx_cp_mail_meeting_send_request;
pub mod wx_cp_mail_schedule_send_request;

pub use wx_cp_mail_common_send_request::Attachment;
pub use wx_cp_mail_common_send_request::BCC;
pub use wx_cp_mail_common_send_request::CC;
pub use wx_cp_mail_common_send_request::TO;
pub use wx_cp_mail_common_send_request::WxCpMailCommonSendRequest;
pub use wx_cp_mail_meeting_send_request::Hosts;
pub use wx_cp_mail_meeting_send_request::Meeting;
pub use wx_cp_mail_meeting_send_request::MeetingAdmins;
pub use wx_cp_mail_meeting_send_request::WxCpMailMeetingSendRequest;
pub use wx_cp_mail_meeting_send_request::WxCpMailMeetingSendRequestOption;
pub use wx_cp_mail_schedule_send_request::Reminders;
pub use wx_cp_mail_schedule_send_request::Schedule;
pub use wx_cp_mail_schedule_send_request::WxCpMailScheduleSendRequest;
