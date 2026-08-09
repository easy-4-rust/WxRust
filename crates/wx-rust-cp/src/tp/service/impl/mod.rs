//! 企业微信第三方应用服务实现（对应 Java `me.chanjar.weixin.cp.tp.service.impl` 包）。

pub mod base_wx_cp_tp_service_impl;
pub mod wx_cp_tp_contact_service_impl;
pub mod wx_cp_tp_customized_service_impl;
pub mod wx_cp_tp_department_service_impl;
pub mod wx_cp_tp_edition_service_impl;
pub mod wx_cp_tp_id_convert_service_impl;
pub mod wx_cp_tp_license_service_impl;
pub mod wx_cp_tp_media_service_impl;
pub mod wx_cp_tp_message_service_impl;
pub mod wx_cp_tp_o_auth2_service_impl;
pub mod wx_cp_tp_oa_service_impl;
pub mod wx_cp_tp_order_service_impl;
pub mod wx_cp_tp_service_impl;
pub mod wx_cp_tp_tag_service_impl;
pub mod wx_cp_tp_user_service_impl;

pub use wx_cp_tp_contact_service_impl::WxCpTpContactServiceImpl;
pub use wx_cp_tp_customized_service_impl::WxCpTpCustomizedServiceImpl;
pub use wx_cp_tp_department_service_impl::WxCpTpDepartmentServiceImpl;
pub use wx_cp_tp_edition_service_impl::WxCpTpEditionServiceImpl;
pub use wx_cp_tp_id_convert_service_impl::WxCpTpIdConvertServiceImpl;
pub use wx_cp_tp_license_service_impl::WxCpTpLicenseServiceImpl;
pub use wx_cp_tp_media_service_impl::WxCpTpMediaServiceImpl;
pub use wx_cp_tp_message_service_impl::WxCpTpMessageServiceImpl;
pub use wx_cp_tp_o_auth2_service_impl::WxCpTpOAuth2ServiceImpl;
pub use wx_cp_tp_oa_service_impl::WxCpTpOAServiceImpl;
pub use wx_cp_tp_order_service_impl::WxCpTpOrderServiceImpl;
pub use wx_cp_tp_service_impl::WxCpTpServiceImpl;
pub use wx_cp_tp_tag_service_impl::WxCpTpTagServiceImpl;
pub use wx_cp_tp_user_service_impl::WxCpTpUserServiceImpl;
