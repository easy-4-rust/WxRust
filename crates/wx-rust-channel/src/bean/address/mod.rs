//! 对应 Java `me.chanjar.weixin.channel.bean.address` 包（生成）。

pub mod address_add_param;
pub mod address_code;
pub mod address_code_response;
pub mod address_detail;
pub mod address_id_param;
pub mod address_id_response;
pub mod address_info_response;
pub mod address_list_param;
pub mod address_list_response;
pub mod offline_address_type;

pub use address_add_param::AddressAddParam;
pub use address_code::AddressCode;
pub use address_code_response::AddressCodeResponse;
pub use address_detail::AddressDetail;
pub use address_id_param::AddressIdParam;
pub use address_id_response::AddressIdResponse;
pub use address_info_response::AddressInfoResponse;
pub use address_list_param::AddressListParam;
pub use address_list_response::AddressListResponse;
pub use offline_address_type::OfflineAddressType;
