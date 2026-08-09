//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// <a href="https://developers.weixin.qq.com/doc/channels/API/leads/get_leads_info_by_component_id.html">按时间获取留资信息详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEADS_INFO_BY_COMPONENT_ID`）。
pub const GET_LEADS_INFO_BY_COMPONENT_ID: &str =
    "https://api.weixin.qq.com/channels/leads/get_leads_info_by_component_id";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/leads/get_leads_info_by_request_id.html">按直播场次获取留资信息详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEADS_INFO_BY_REQUEST_ID`）。
pub const GET_LEADS_INFO_BY_REQUEST_ID: &str =
    "https://api.weixin.qq.com/channels/leads/get_leads_info_by_request_id";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/leads/get_leads_request_id.html">获取留资request_id列表详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEADS_REQUEST_ID`）。
pub const GET_LEADS_REQUEST_ID: &str =
    "https://api.weixin.qq.com/channels/leads/get_leads_request_id";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/leads/get_leads_component_promote_record.html">获取留资组件直播推广记录信息详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEADS_COMPONENT_PROMOTE_RECORD`）。
pub const GET_LEADS_COMPONENT_PROMOTE_RECORD: &str =
    "https://api.weixin.qq.com/channels/leads/get_leads_component_promote_record";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/leads/get_leads_component_id.html">获取留资组件Id列表详情</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEADS_COMPONENT_ID`）。
pub const GET_LEADS_COMPONENT_ID: &str =
    "https://api.weixin.qq.com/channels/leads/get_leads_component_id";
