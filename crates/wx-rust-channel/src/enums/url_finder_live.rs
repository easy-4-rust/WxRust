//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// <a href="https://developers.weixin.qq.com/doc/channels/API/live/get_finder_attr_by_appid.html">获取视频号账号信息</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_ATTR_BY_APPID`）。
pub const GET_FINDER_ATTR_BY_APPID: &str =
    "https://api.weixin.qq.com/channels/finderlive/get_finder_attr_by_appid";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/live/get_finder_live_data_list.html">获取留资直播间数据详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_LIVE_DATA_LIST`）。
pub const GET_FINDER_LIVE_DATA_LIST: &str =
    "https://api.weixin.qq.com/channels/finderlive/get_finder_live_data_list";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/live/get_finder_live_leads_data.html">获取账号收集的留资数量</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_FINDER_LIVE_LEADS_DATA`）。
pub const GET_FINDER_LIVE_LEADS_DATA: &str =
    "https://api.weixin.qq.com/channels/finderlive/get_finder_live_leads_data";
