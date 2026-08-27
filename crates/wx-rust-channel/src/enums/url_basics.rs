//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取店铺基本信息（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_INFO`）。
pub const GET_SHOP_INFO: &str = "https://api.weixin.qq.com/channels/ec/basics/info/get";

/// 上传图片（对应 Java `WxChannelApiUrlConstants` 常量 `IMG_UPLOAD_URL`）。
pub const IMG_UPLOAD_URL: &str = "https://api.weixin.qq.com/shop/ec/basics/img/upload";

/// 上传资质图片（对应 Java `WxChannelApiUrlConstants` 常量 `UPLOAD_QUALIFICATION_FILE`）。
pub const UPLOAD_QUALIFICATION_FILE: &str =
    "https://api.weixin.qq.com/shop/ec/basics/qualification/upload";

/// 下载图片（对应 Java `WxChannelApiUrlConstants` 常量 `GET_IMG_URL`）。
pub const GET_IMG_URL: &str = "https://api.weixin.qq.com/channels/ec/basics/media/get";

/// 获取地址编码（对应 Java `WxChannelApiUrlConstants` 常量 `GET_ADDRESS_CODE`）。
pub const GET_ADDRESS_CODE: &str = "https://api.weixin.qq.com/channels/ec/basics/addresscode/get";

/// 获取店铺 H5 链接（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_H5URL`）。
pub const GET_SHOP_H5URL: &str = "https://api.weixin.qq.com/channels/ec/basics/shop/h5url/get";

/// 获取店铺二维码（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_QRCODE`）。
pub const GET_SHOP_QRCODE: &str = "https://api.weixin.qq.com/channels/ec/basics/shop/qrcode/get";

/// 获取店铺口令（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOP_TAGLINK`）。
pub const GET_SHOP_TAGLINK: &str = "https://api.weixin.qq.com/channels/ec/basics/shop/taglink/get";
