//! 接口结果数据对象。
//!
//! 对应 Java `me.chanjar.weixin.common.bean.result` 包。

pub mod wx_media_upload_result;
pub mod wx_minishop_image_upload_customize_result;
pub mod wx_minishop_image_upload_result;
pub mod wx_minishop_pic_file_customize_result;
pub mod wx_minishop_pic_file_result;

pub use wx_media_upload_result::WxMediaUploadResult;
pub use wx_minishop_image_upload_customize_result::WxMinishopImageUploadCustomizeResult;
pub use wx_minishop_image_upload_result::WxMinishopImageUploadResult;
pub use wx_minishop_pic_file_customize_result::WxMinishopPicFileCustomizeResult;
pub use wx_minishop_pic_file_result::WxMinishopPicFileResult;
