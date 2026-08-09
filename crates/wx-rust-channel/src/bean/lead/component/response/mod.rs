//! 对应 Java `me.chanjar.weixin.channel.bean.lead/component/response` 包（生成）。

pub mod finder_attr_response;
pub mod get_finder_live_data_list_response;
pub mod get_finder_live_leads_data_response;
pub mod get_leads_component_id_response;
pub mod get_leads_component_promote_record_response;
pub mod get_leads_request_id_response;
pub mod lead_info_response;

pub use finder_attr_response::FinderAttr;
pub use finder_attr_response::FinderAttrResponse;
pub use get_finder_live_data_list_response::GetFinderLiveDataListResponse;
pub use get_finder_live_data_list_response::LiveStatisticsItem;
pub use get_finder_live_leads_data_response::GetFinderLiveLeadsDataResponse;
pub use get_finder_live_leads_data_response::LeadCountItem;
pub use get_leads_component_id_response::GetLeadsComponentIdResponse;
pub use get_leads_component_id_response::LeadComponentItem;
pub use get_leads_component_promote_record_response::GetLeadsComponentPromoteRecordResponse;
pub use get_leads_component_promote_record_response::RecordData;
pub use get_leads_request_id_response::GetLeadsRequestIdResponse;
pub use get_leads_request_id_response::LiveLeadItem;
pub use lead_info_response::LeadInfoResponse;
pub use lead_info_response::LeadsData;
pub use lead_info_response::UserData;
