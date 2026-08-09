//! 对应 Java `cn.binarywang.wx.miniapp.bean.kefu` 包（生成）。

pub mod request;
pub mod wx_ma_kf_info;
pub mod wx_ma_kf_list;
pub mod wx_ma_kf_session;
pub mod wx_ma_kf_session_list;

pub use request::wx_ma_kf_account_request::WxMaKfAccountRequest;
pub use request::wx_ma_kf_session_request::WxMaKfSessionRequest;
pub use wx_ma_kf_info::WxMaKfInfo;
pub use wx_ma_kf_list::WxMaKfList;
pub use wx_ma_kf_session::WxMaKfSession;
pub use wx_ma_kf_session_list::SessionInfo;
pub use wx_ma_kf_session_list::WxMaKfSessionList;
