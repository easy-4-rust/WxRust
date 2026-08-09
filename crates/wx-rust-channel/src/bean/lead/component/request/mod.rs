//! 对应 Java `me.chanjar.weixin.channel.bean.lead/component/request` 包（生成）。

pub mod get_finder_live_data_list_request;
pub mod get_finder_live_leads_data_request;
pub mod get_lead_info_by_component_request;
pub mod get_leads_component_id_request;
pub mod get_leads_component_promote_record_request;
pub mod get_leads_info_by_request_id_request;
pub mod get_leads_request_id_request;

pub use get_finder_live_data_list_request::GetFinderLiveDataListRequest;
pub use get_finder_live_leads_data_request::GetFinderLiveLeadsDataRequest;
pub use get_lead_info_by_component_request::GetLeadInfoByComponentRequest;
pub use get_leads_component_id_request::GetLeadsComponentIdRequest;
pub use get_leads_component_promote_record_request::GetLeadsComponentPromoteRecordRequest;
pub use get_leads_info_by_request_id_request::GetLeadsInfoByRequestIdRequest;
pub use get_leads_request_id_request::GetLeadsRequestIdRequest;
