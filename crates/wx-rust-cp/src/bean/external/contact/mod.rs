//! 对应 Java `me.chanjar.weixin.cp.bean.external/contact` 包（生成）。

pub mod external_contact;
pub mod followed_user;
pub mod wx_cp_external_contact_batch_info;
pub mod wx_cp_external_contact_info;
pub mod wx_cp_external_contact_list_info;
pub mod wx_cp_group_msg_list_result;
pub mod wx_cp_group_msg_result;
pub mod wx_cp_group_msg_send_result;
pub mod wx_cp_group_msg_task_result;

pub use external_contact::ExternalAttribute;
pub use external_contact::ExternalContact;
pub use external_contact::ExternalProfile;
pub use external_contact::MiniProgram;
pub use external_contact::Text;
pub use external_contact::Web;
pub use external_contact::WechatChannel;
pub use followed_user::FollowedUser;
pub use followed_user::Tag;
pub use followed_user::WechatChannels;
pub use wx_cp_external_contact_batch_info::ExternalContactInfo;
pub use wx_cp_external_contact_batch_info::WxCpExternalContactBatchInfo;
pub use wx_cp_external_contact_info::WxCpExternalContactInfo;
pub use wx_cp_external_contact_list_info::WxCpExternalContactListInfo;
pub use wx_cp_group_msg_list_result::ExternalContactGroupMsgInfo;
pub use wx_cp_group_msg_list_result::WxCpGroupMsgListResult;
pub use wx_cp_group_msg_result::ExternalContactGroupMsgDetailInfo;
pub use wx_cp_group_msg_result::WxCpGroupMsgResult;
pub use wx_cp_group_msg_send_result::ExternalContactGroupMsgSendInfo;
pub use wx_cp_group_msg_send_result::WxCpGroupMsgSendResult;
pub use wx_cp_group_msg_task_result::ExternalContactGroupMsgTaskInfo;
pub use wx_cp_group_msg_task_result::WxCpGroupMsgTaskResult;
