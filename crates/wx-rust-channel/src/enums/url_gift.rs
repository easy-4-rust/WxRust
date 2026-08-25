//! 视频号小店赠品活动接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 添加非卖商品（对应 Java `GIFT_PRODUCT_ADD_URL`）。
pub const GIFT_PRODUCT_ADD_URL: &str = "https://api.weixin.qq.com/channels/ec/product/gift/add";

/// 更新非卖商品（对应 Java `GIFT_PRODUCT_UPDATE_URL`）。
pub const GIFT_PRODUCT_UPDATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/gift/update";

/// 在售商品转赠品（对应 Java `GIFT_PRODUCT_ON_SALE_SET_URL`）。
pub const GIFT_PRODUCT_ON_SALE_SET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/gift/onsale/set";

/// 获取赠品（对应 Java `GIFT_PRODUCT_GET_URL`）。
pub const GIFT_PRODUCT_GET_URL: &str = "https://api.weixin.qq.com/channels/ec/product/gift/get";

/// 获取赠品列表（对应 Java `GIFT_PRODUCT_LIST_URL`）。
pub const GIFT_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/gift/list/get";

/// 更新赠品库存（对应 Java `GIFT_PRODUCT_STOCK_UPDATE_URL`）。
pub const GIFT_PRODUCT_STOCK_UPDATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/gift/stock/update";

/// 创建赠品活动（对应 Java `GIFT_ACTIVITY_ADD_URL`）。
pub const GIFT_ACTIVITY_ADD_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/activity/add";

/// 删除赠品活动（对应 Java `GIFT_ACTIVITY_DELETE_URL`）。
pub const GIFT_ACTIVITY_DELETE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/activity/del";

/// 停止赠品活动（对应 Java `GIFT_ACTIVITY_STOP_URL`）。
pub const GIFT_ACTIVITY_STOP_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/activity/stop";
