//! 对应 Java `me.chanjar.weixin.open.bean.ma/privacy` 包（生成）。

pub mod apply_privacy_interface;
pub mod apply_privacy_interface_result;
pub mod get_privacy_interface_result;
pub mod get_privacy_setting_result;
pub mod privacy_key_enum;
pub mod privacy_owner_setting;
pub mod set_privacy_setting;
pub mod upload_privacy_file_result;

pub use apply_privacy_interface::ApplyPrivacyInterface;
pub use apply_privacy_interface_result::ApplyPrivacyInterfaceResult;
pub use get_privacy_interface_result::GetPrivacyInterfaceResult;
pub use get_privacy_interface_result::Interface;
pub use get_privacy_setting_result::GetPrivacySettingResult;
pub use get_privacy_setting_result::PrivacyDesc;
pub use get_privacy_setting_result::PrivacyDescItem;
pub use get_privacy_setting_result::Setting;
pub use privacy_key_enum::PrivacyKeyEnum;
pub use privacy_owner_setting::PrivacyOwnerSetting;
pub use set_privacy_setting::SetPrivacySetting;
pub use upload_privacy_file_result::UploadPrivacyFileResult;
