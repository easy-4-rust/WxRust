//! 对应 Java `cn.binarywang.wx.miniapp.bean.complaint` 包（生成）。

pub mod wx_ma_complaint_detail_request;
pub mod wx_ma_complaint_detail_result;
pub mod wx_ma_complaint_notify_url_request;
pub mod wx_ma_complaint_notify_url_result;
pub mod wx_ma_complaint_request;
pub mod wx_ma_complaint_result;
pub mod wx_ma_complete_request;
pub mod wx_ma_negotiation_history_request;
pub mod wx_ma_negotiation_history_result;
pub mod wx_ma_response_request;

pub use wx_ma_complaint_detail_request::WxMaComplaintDetailRequest;
pub use wx_ma_complaint_detail_result::ComplaintMedia;
pub use wx_ma_complaint_detail_result::ComplaintOrderInfo;
pub use wx_ma_complaint_detail_result::WxMaComplaintDetailResult;
pub use wx_ma_complaint_notify_url_request::WxMaComplaintNotifyUrlRequest;
pub use wx_ma_complaint_notify_url_result::WxMaComplaintNotifyUrlResult;
pub use wx_ma_complaint_request::WxMaComplaintRequest;
pub use wx_ma_complaint_result::Complaint;
pub use wx_ma_complaint_result::WxMaComplaintResult;
pub use wx_ma_complete_request::WxMaCompleteRequest;
pub use wx_ma_negotiation_history_request::WxMaNegotiationHistoryRequest;
pub use wx_ma_negotiation_history_result::NegotiationHistory;
pub use wx_ma_negotiation_history_result::WxMaNegotiationHistoryResult;
pub use wx_ma_response_request::WxMaResponseRequest;
