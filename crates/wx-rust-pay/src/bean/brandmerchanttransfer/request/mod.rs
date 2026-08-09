//! 对应 Java `com.github.binarywang.wxpay.bean.brandmerchanttransfer/request` 包（生成）。

pub mod brand_merchant_batches_query_request;
pub mod brand_merchant_details_query_request;
pub mod brand_transfer_batches_request;
pub mod brand_wx_batches_query_request;
pub mod brand_wx_details_query_request;

pub use brand_merchant_batches_query_request::BrandMerchantBatchesQueryRequest;
pub use brand_merchant_details_query_request::BrandMerchantDetailsQueryRequest;
pub use brand_transfer_batches_request::BrandTransferBatchesRequest;
pub use brand_transfer_batches_request::BrandTransferDetail;
pub use brand_wx_batches_query_request::BrandWxBatchesQueryRequest;
pub use brand_wx_details_query_request::BrandWxDetailsQueryRequest;
