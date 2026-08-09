//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// <a href="https://developers.weixin.qq.com/doc/channels/API/compass/finder/getfinderoverall.html">获取电商概览数据</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_OVERALL_URL`）。
pub const GET_OVERALL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/finder/overall/get";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/compass/finder/getfinderproductdata.html">获取带货商品数据</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PRODUCT_DATA_URL`）。
pub const GET_PRODUCT_DATA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/finder/product/data/get";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/compass/finder/getfinderproductlist.html">获取带货商品列表</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PRODUCT_LIST_URL`）。
pub const GET_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/finder/product/list/get";

/// <a href="https://developers.weixin.qq.com/doc/channels/API/compass/finder/getfindersaleprofiledata.html">获取带货人群数据</a>（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SALE_PROFILE_DATA_URL`）。
pub const GET_SALE_PROFILE_DATA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/compass/finder/sale/profile/data/get";
