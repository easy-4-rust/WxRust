//! 对应 Java `me.chanjar.weixin.channel.bean.lead` 包（生成）。

pub mod component;

pub use component::request::get_finder_live_data_list_request::GetFinderLiveDataListRequest;
pub use component::request::get_finder_live_leads_data_request::GetFinderLiveLeadsDataRequest;
pub use component::request::get_lead_info_by_component_request::GetLeadInfoByComponentRequest;
pub use component::request::get_leads_component_id_request::GetLeadsComponentIdRequest;
pub use component::request::get_leads_component_promote_record_request::GetLeadsComponentPromoteRecordRequest;
pub use component::request::get_leads_info_by_request_id_request::GetLeadsInfoByRequestIdRequest;
pub use component::request::get_leads_request_id_request::GetLeadsRequestIdRequest;
pub use component::response::finder_attr_response::FinderAttr;
pub use component::response::finder_attr_response::FinderAttrResponse;
pub use component::response::get_finder_live_data_list_response::GetFinderLiveDataListResponse;
pub use component::response::get_finder_live_data_list_response::LiveStatisticsItem;
pub use component::response::get_finder_live_leads_data_response::GetFinderLiveLeadsDataResponse;
pub use component::response::get_finder_live_leads_data_response::LeadCountItem;
pub use component::response::get_leads_component_id_response::GetLeadsComponentIdResponse;
pub use component::response::get_leads_component_id_response::LeadComponentItem;
pub use component::response::get_leads_component_promote_record_response::GetLeadsComponentPromoteRecordResponse;
pub use component::response::get_leads_component_promote_record_response::RecordData;
pub use component::response::get_leads_request_id_response::GetLeadsRequestIdResponse;
pub use component::response::get_leads_request_id_response::LiveLeadItem;
pub use component::response::lead_info_response::LeadInfoResponse;
pub use component::response::lead_info_response::LeadsData;
pub use component::response::lead_info_response::UserData;
