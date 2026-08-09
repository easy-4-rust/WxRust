//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer` 包（生成）。

pub mod request;
pub mod result;

pub use request::brand_merchant_batches_query_request::BrandMerchantBatchesQueryRequest;
pub use request::brand_merchant_details_query_request::BrandMerchantDetailsQueryRequest;
pub use request::brand_transfer_batches_request::BrandTransferBatchesRequest;
pub use request::brand_transfer_batches_request::BrandTransferDetail;
pub use request::brand_wx_batches_query_request::BrandWxBatchesQueryRequest;
pub use request::brand_wx_details_query_request::BrandWxDetailsQueryRequest;
pub use result::brand_batches_query_result::BrandBatchesQueryResult;
pub use result::brand_batches_query_result::BrandDetailResult;
pub use result::brand_details_query_result::BrandDetailsQueryResult;
pub use result::brand_transfer_batches_result::BrandTransferBatchesResult;
