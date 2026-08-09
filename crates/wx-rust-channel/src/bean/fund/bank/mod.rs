//! 对应 Java `me.chanjar.weixin.channel.bean.fund/bank` 包（生成）。

pub mod bank_city_info;
pub mod bank_city_response;
pub mod bank_info;
pub mod bank_info_response;
pub mod bank_list_response;
pub mod bank_province_info;
pub mod bank_province_response;
pub mod bank_search_param;
pub mod branch_info;
pub mod branch_info_response;
pub mod branch_search_param;

pub use bank_city_info::BankCityInfo;
pub use bank_city_response::BankCityResponse;
pub use bank_info::BankInfo;
pub use bank_info_response::BankInfoResponse;
pub use bank_list_response::BankListResponse;
pub use bank_province_info::BankProvinceInfo;
pub use bank_province_response::BankProvinceResponse;
pub use bank_search_param::BankSearchParam;
pub use branch_info::BranchInfo;
pub use branch_info_response::BranchInfoResponse;
pub use branch_search_param::BranchSearchParam;
