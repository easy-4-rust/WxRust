//! 对应 Java `me.chanjar.weixin.open.bean.message` 包（生成）。

pub mod wx_open_ma_submit_audit_message;
pub mod wx_open_ma_verify_beta_weapp_message;
pub mod wx_open_xml_message;

pub use wx_open_ma_submit_audit_message::WxMaCodeSubmitAuditItem;
pub use wx_open_ma_submit_audit_message::WxMaCodeSubmitAuditPreviewInfo;
pub use wx_open_ma_submit_audit_message::WxOpenMaSubmitAuditMessage;
pub use wx_open_ma_verify_beta_weapp_message::WxOpenMaVerifyBetaWeappMessage;
pub use wx_open_xml_message::DispatchInfo;
pub use wx_open_xml_message::Info;
pub use wx_open_xml_message::WxOpenXmlMessage;
