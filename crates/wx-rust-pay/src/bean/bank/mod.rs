//! 对应 Java `com.github.binarywang.wxpay.bean.bank` 包（生成）。

pub mod bank_account_result;
pub mod bank_branches_result;
pub mod bank_info;
pub mod banking_result;
pub mod cities_result;
pub mod page_link;
pub mod provinces_result;

pub use bank_account_result::BankAccountResult;
pub use bank_branches_result::BankBranch;
pub use bank_branches_result::BankBranchesResult;
pub use bank_info::BankInfo;
pub use banking_result::BankingResult;
pub use banking_result::Link;
pub use cities_result::CitiesResult;
pub use cities_result::CityInfo;
pub use page_link::PageLink;
pub use provinces_result::ProvinceInfo;
pub use provinces_result::ProvincesResult;
