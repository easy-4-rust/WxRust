//! 素材（媒体）相关接口地址。
//!
//! 对应 Java `WxCpApiPathConsts.Media`。

/// 获取素材。
pub const MEDIA_GET: &str = "/cgi-bin/media/get";
/// 上传素材（`type` 拼在路径后）。
pub const MEDIA_UPLOAD: &str = "/cgi-bin/media/upload?type=";
/// 上传图片。
pub const IMG_UPLOAD: &str = "/cgi-bin/media/uploadimg";
/// 获取高清语音素材（JSSDK）。
pub const JSSDK_MEDIA_GET: &str = "/cgi-bin/media/get/jssdk";
/// 获取通过 URL 上传素材的结果。
pub const GET_UPLOAD_BY_URL_RESULT: &str = "/cgi-bin/media/get_upload_by_url_result";
/// 通过 URL 上传素材。
pub const UPLOAD_BY_URL: &str = "/cgi-bin/media/upload_by_url";
