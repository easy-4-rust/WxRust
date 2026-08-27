//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 添加商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_ADD_URL`）。
pub const SPU_ADD_URL: &str = "https://api.weixin.qq.com/channels/ec/product/add";

/// 删除商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_DEL_URL`）。
pub const SPU_DEL_URL: &str = "https://api.weixin.qq.com/channels/ec/product/delete";

/// 获取商品详情（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_GET_URL`）。
pub const SPU_GET_URL: &str = "https://api.weixin.qq.com/channels/ec/product/get";

/// 获取商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_LIST_URL`）。
pub const SPU_LIST_URL: &str = "https://api.weixin.qq.com/channels/ec/product/list/get";

/// 更新商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_UPDATE_URL`）。
pub const SPU_UPDATE_URL: &str = "https://api.weixin.qq.com/channels/ec/product/update";

/// 更新商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_AUDIT_FREE_UPDATE_URL`）。
pub const SPU_AUDIT_FREE_UPDATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/auditfree";

/// 上架商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_LISTING_URL`）。
pub const SPU_LISTING_URL: &str = "https://api.weixin.qq.com/channels/ec/product/listing";

/// 下架商品（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_DELISTING_URL`）。
pub const SPU_DELISTING_URL: &str = "https://api.weixin.qq.com/channels/ec/product/delisting";

/// 撤回商品审核（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_AUDIT_URL`）。
pub const CANCEL_AUDIT_URL: &str = "https://api.weixin.qq.com/channels/ec/product/audit/cancel";

/// 获取商品H5短链（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_H5URL_URL`）。
pub const SPU_H5URL_URL: &str = "https://api.weixin.qq.com/channels/ec/product/h5url/get";

/// 获取商品二维码（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_QRCODE_URL`）。
pub const SPU_QRCODE_URL: &str = "https://api.weixin.qq.com/channels/ec/product/qrcode/get";

/// 获取商品口令（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_TAGLINK_URL`）。
pub const SPU_TAGLINK_URL: &str = "https://api.weixin.qq.com/channels/ec/product/taglink/get";

/// 获取实时库存（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_GET_STOCK_URL`）。
pub const SPU_GET_STOCK_URL: &str = "https://api.weixin.qq.com/channels/ec/product/stock/get";

/// 获取实时库存（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_GET_STOCK_BATCH_URL`）。
pub const SPU_GET_STOCK_BATCH_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/stock/batchget";

/// 更新商品库存（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_UPDATE_STOCK_URL`）。
pub const SPU_UPDATE_STOCK_URL: &str = "https://api.weixin.qq.com/channels/ec/product/stock/update";

/// 添加限时抢购任务（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_LIMIT_TASK_URL`）。
pub const ADD_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/add";

/// 拉取限时抢购任务列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_LIMIT_TASK_URL`）。
pub const LIST_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/list/get";

/// 停止限时抢购任务（对应 Java `WxChannelApiUrlConstants` 常量 `STOP_LIMIT_TASK_URL`）。
pub const STOP_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/stop";

/// 删除限时抢购任务（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_LIMIT_TASK_URL`）。
pub const DELETE_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/delete";

/// 获取商品移动应用跳转 scheme 码（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_SCHEME_URL`）。
pub const SPU_SCHEME_URL: &str = "https://api.weixin.qq.com/channels/ec/product/scheme/get";

/// 商品类目推荐（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_CATEGORY_CLASSIFY_URL`）。
pub const SPU_CATEGORY_CLASSIFY_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/category/classify";

/// 商品定时开售（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_BEGIN_TIMING_SALE_URL`）。
pub const SPU_BEGIN_TIMING_SALE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/begintimingsale";

/// 取消商品定时开售（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_CANCEL_TIMING_SALE_URL`）。
pub const SPU_CANCEL_TIMING_SALE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/canceltimingsale";

/// 外部商品映射（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_EXTERNAL_PRODUCT_MAPPING_URL`）。
pub const SPU_EXTERNAL_PRODUCT_MAPPING_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/externalproductmapping";

/// 类目预检（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_CATEGORY_PRE_CHECK_URL`）。
pub const SPU_CATEGORY_PRE_CHECK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/categoryprecheck";

/// 获取商品上架策略（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_AUDIT_STRATEGY_GET_URL`）。
pub const SPU_AUDIT_STRATEGY_GET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/auditstrategy/get";

/// 设置商品上架策略（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_AUDIT_STRATEGY_SET_URL`）。
pub const SPU_AUDIT_STRATEGY_SET_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/auditstrategy/set";

/// 获取商品提审限额（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_GET_AUDIT_QUOTA_URL`）。
pub const SPU_GET_AUDIT_QUOTA_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/getauditquota";

/// 外部商品映射（新版）（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL`）。
pub const SPU_EXTERNAL_PRODUCT_MAPPING_NEW_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/externalproductmappingnew";

/// 商品品牌推荐（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_PRODUCT_BRAND_RECOMMEND_URL`）。
pub const SPU_PRODUCT_BRAND_RECOMMEND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/productbrandrecommend";

/// 新增第三方货源信息（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL`）。
pub const SPU_ADD_PRODUCT_THIRD_PARTY_SOURCE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/addproductthirdpartysource";

/// 获取库存流水（对应 Java `WxChannelApiUrlConstants` 常量 `SPU_GET_STOCK_FLOW_URL`）。
pub const SPU_GET_STOCK_FLOW_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/stock/getflow";
