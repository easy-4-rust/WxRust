//! 对应 Java `me.chanjar.weixin.mp.bean.device` 包（生成）。

pub mod abstract_device_bean;
pub mod base_resp;
pub mod resp_msg;
pub mod trans_msg_resp;
pub mod wx_device;
pub mod wx_device_authorize;
pub mod wx_device_authorize_result;
pub mod wx_device_bind;
pub mod wx_device_bind_device_result;
pub mod wx_device_bind_result;
pub mod wx_device_msg;
pub mod wx_device_open_id_result;
pub mod wx_device_qr_code_result;

pub use abstract_device_bean::AbstractDeviceBean;
pub use base_resp::BaseInfo;
pub use base_resp::BaseResp;
pub use resp_msg::RespMsg;
pub use trans_msg_resp::TransMsgResp;
pub use wx_device::WxDevice;
pub use wx_device_authorize::WxDeviceAuthorize;
pub use wx_device_authorize_result::WxDeviceAuthorizeResult;
pub use wx_device_bind::WxDeviceBind;
pub use wx_device_bind_device_result::Device;
pub use wx_device_bind_device_result::WxDeviceBindDeviceResult;
pub use wx_device_bind_result::WxDeviceBindResult;
pub use wx_device_msg::WxDeviceMsg;
pub use wx_device_open_id_result::WxDeviceOpenIdResult;
pub use wx_device_qr_code_result::WxDeviceQrCodeResult;
