//! 对应 Java `com.github.binarywang.wxpay.bean.marketing/transfer` 包（生成）。

pub mod batch_details_request;
pub mod batch_details_result;
pub mod batch_number_request;
pub mod batch_number_result;
pub mod bill_receipt_result;
pub mod download_request;
pub mod electronic_receipts_request;
pub mod electronic_receipts_result;
pub mod merchant_batch_request;
pub mod partner_transfer_request;
pub mod partner_transfer_result;
pub mod receipt_bill_request;

pub use batch_details_request::BatchDetailsRequest;
pub use batch_details_result::BatchDetailsResult;
pub use batch_number_request::BatchNumberRequest;
pub use batch_number_result::BatchNumberResult;
pub use batch_number_result::TransferDetail;
pub use bill_receipt_result::BillReceiptResult;
pub use download_request::DownloadRequest;
pub use electronic_receipts_request::ElectronicReceiptsRequest;
pub use electronic_receipts_result::ElectronicReceiptsResult;
pub use merchant_batch_request::MerchantBatchRequest;
pub use partner_transfer_request::PartnerTransferRequest;
pub use partner_transfer_result::PartnerTransferResult;
pub use receipt_bill_request::ReceiptBillRequest;
