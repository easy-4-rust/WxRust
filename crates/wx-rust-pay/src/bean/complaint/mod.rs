//! 对应 Java `com.github.binarywang.wxpay.bean.complaint` 包（生成）。

pub mod complaint_detail_request;
pub mod complaint_detail_result;
pub mod complaint_notify_url_request;
pub mod complaint_notify_url_result;
pub mod complaint_request;
pub mod complaint_result;
pub mod complete_request;
pub mod negotiation_history_request;
pub mod negotiation_history_result;
pub mod response_request;
pub mod update_refund_progress_request;

pub use complaint_detail_request::ComplaintDetailRequest;
pub use complaint_detail_result::AdditionalInfo;
pub use complaint_detail_result::ComplaintDetailResult;
pub use complaint_detail_result::ComplaintMedia;
pub use complaint_detail_result::ComplaintOrder;
pub use complaint_detail_result::ReturnAddressInfo;
pub use complaint_detail_result::ServiceOrder;
pub use complaint_detail_result::SharePowerInfo;
pub use complaint_notify_url_request::ComplaintNotifyUrlRequest;
pub use complaint_notify_url_result::ComplaintNotifyUrlResult;
pub use complaint_request::ComplaintRequest;
pub use complaint_result::ComplaintResult;
pub use complete_request::CompleteRequest;
pub use negotiation_history_request::NegotiationHistoryRequest;
pub use negotiation_history_result::NegotiationHistory;
pub use negotiation_history_result::NegotiationHistoryResult;
pub use response_request::MiniProgramJumpInfo;
pub use response_request::ResponseRequest;
pub use update_refund_progress_request::UpdateRefundProgressRequest;
