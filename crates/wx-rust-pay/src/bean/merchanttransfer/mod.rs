//! 对应 Java `com.github.binarywang.wxpay.bean.merchanttransfer` 包（生成）。

pub mod batches_query_result;
pub mod detail_electronic_bill_request;
pub mod detail_electronic_bill_result;
pub mod details_query_result;
pub mod electronic_bill_apply_request;
pub mod electronic_bill_result;
pub mod merchant_batches_query_request;
pub mod merchant_details_query_request;
pub mod transfer_create_request;
pub mod transfer_create_result;
pub mod wx_batches_query_request;
pub mod wx_details_query_request;

pub use batches_query_result::BatchesQueryResult;
pub use batches_query_result::TransferBatch;
pub use batches_query_result::TransferDetail;
pub use detail_electronic_bill_request::DetailElectronicBillRequest;
pub use detail_electronic_bill_result::DetailElectronicBillResult;
pub use details_query_result::DetailsQueryResult;
pub use electronic_bill_apply_request::ElectronicBillApplyRequest;
pub use electronic_bill_result::ElectronicBillResult;
pub use merchant_batches_query_request::MerchantBatchesQueryRequest;
pub use merchant_details_query_request::MerchantDetailsQueryRequest;
pub use transfer_create_request::TransferCreateRequest;
pub use transfer_create_request::TransferDetailList;
pub use transfer_create_result::TransferCreateResult;
pub use wx_batches_query_request::WxBatchesQueryRequest;
pub use wx_details_query_request::WxDetailsQueryRequest;
