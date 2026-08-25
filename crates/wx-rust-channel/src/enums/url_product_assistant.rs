//! 视频号小店商品辅助功能接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 发品前校验（对应 Java `CATEGORY_PRE_CHECK_URL`）。
pub const CATEGORY_PRE_CHECK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/categoryprecheck";

/// 获取商品品牌推荐（对应 Java `PRODUCT_BRAND_RECOMMEND_URL`）。
pub const PRODUCT_BRAND_RECOMMEND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/productbrandrecommend";

/// 获取站内外商品属性映射（对应 Java `EXTERNAL_PRODUCT_MAPPING_URL`）。
pub const EXTERNAL_PRODUCT_MAPPING_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/externalproductmapping";

/// 获取商品属性映射及推荐（对应 Java `EXTERNAL_PRODUCT_MAPPING_NEW_URL`）。
pub const EXTERNAL_PRODUCT_MAPPING_NEW_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/externalproductmappingnew";

/// 将定时开售商品改为立即开售（对应 Java `BEGIN_TIMING_SALE_URL`）。
pub const BEGIN_TIMING_SALE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/begintimingsale";

/// 取消商品定时开售（对应 Java `CANCEL_TIMING_SALE_URL`）。
pub const CANCEL_TIMING_SALE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/canceltimingsale";
