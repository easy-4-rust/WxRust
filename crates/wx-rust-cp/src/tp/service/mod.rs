//! 企业微信第三方应用（服务商）服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.tp.service` 包：`WxCpTpService` 门面 +
//! 14 个子服务接口（contact/customized/department/edition/id_convert/
//! license/media/message/oa/oauth2/order/tag/user + `WxCpTpService` 自身）。

pub mod wx_cp_tp_contact_service;
pub mod wx_cp_tp_customized_service;
pub mod wx_cp_tp_department_service;
pub mod wx_cp_tp_edition_service;
pub mod wx_cp_tp_id_convert_service;
pub mod wx_cp_tp_license_service;
pub mod wx_cp_tp_media_service;
pub mod wx_cp_tp_message_service;
pub mod wx_cp_tp_o_auth2_service;
pub mod wx_cp_tp_oa_service;
pub mod wx_cp_tp_order_service;
pub mod wx_cp_tp_service;
pub mod wx_cp_tp_tag_service;
pub mod wx_cp_tp_user_service;

pub mod r#impl;

pub use wx_cp_tp_contact_service::WxCpTpContactService;
pub use wx_cp_tp_customized_service::WxCpTpCustomizedService;
pub use wx_cp_tp_department_service::WxCpTpDepartmentService;
pub use wx_cp_tp_edition_service::WxCpTpEditionService;
pub use wx_cp_tp_id_convert_service::WxCpTpIdConvertService;
pub use wx_cp_tp_license_service::WxCpTpLicenseService;
pub use wx_cp_tp_media_service::WxCpTpMediaService;
pub use wx_cp_tp_message_service::WxCpTpMessageService;
pub use wx_cp_tp_o_auth2_service::WxCpTpOAuth2Service;
pub use wx_cp_tp_oa_service::WxCpTpOAService;
pub use wx_cp_tp_order_service::WxCpTpOrderService;
pub use wx_cp_tp_service::WxCpTpService;
pub use wx_cp_tp_tag_service::WxCpTpTagService;
pub use wx_cp_tp_user_service::WxCpTpUserService;
