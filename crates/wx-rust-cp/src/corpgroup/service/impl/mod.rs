//! 企业互联服务实现（对应 Java
//! `me.chanjar.weixin.cp.corpgroup.service.impl` 包）。

pub mod base_wx_cp_cg_service_impl;
pub mod wx_cp_linked_corp_service_impl;

pub use base_wx_cp_cg_service_impl::WxCpCgServiceImpl;
pub use wx_cp_linked_corp_service_impl::WxCpLinkedCorpServiceImpl;
