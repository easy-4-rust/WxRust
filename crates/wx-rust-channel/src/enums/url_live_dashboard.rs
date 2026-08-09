//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// <a href="https://developers.weixin.qq.com/doc/channels/API/livedashboard/getlivelist.html">获取直播大屏直播列表</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LIVE_LIST_URL`）。
pub const GET_LIVE_LIST_URL: &str = "https://api.weixin.qq.com/channels/livedashboard/getlivelist";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/livedashboard/getlivedata.html">获取直播大屏数据</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LIVE_DATA_URL`）。
pub const GET_LIVE_DATA_URL: &str = "https://api.weixin.qq.com/channels/livedashboard/getlivedata";
