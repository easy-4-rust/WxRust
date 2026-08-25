//! 视频号小店代发管理接口地址常量（对应 Java `WxChannelApiUrlConstants.Supplier`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 获取供货商列表（对应 Java `Supplier.GET_SUPPLIER_LIST_URL`）。
pub const GET_SUPPLIER_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/get_supplier_list";

/// 获取分配方式（对应 Java `Supplier.GET_DISTRIBUTE_URL`）。
pub const GET_DISTRIBUTE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/get_distribute";

/// 设置全店订单手动分配（对应 Java `Supplier.SET_MANUALLY_DISTRIBUTE_URL`）。
pub const SET_MANUALLY_DISTRIBUTE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/set_manually_distribute";

/// 设置全店订单自动分配（对应 Java `Supplier.SET_ALL_DISTRIBUTION_URL`）。
pub const SET_ALL_DISTRIBUTION_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/set_all_distribution";

/// 设置按商品自动分配（对应 Java `Supplier.SET_PRODUCT_DISTRIBUTE_URL`）。
pub const SET_PRODUCT_DISTRIBUTE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/set_product_distribute";

/// 获取商品对应的自动分配供货商（对应 Java `Supplier.GET_PRODUCT_DEFAULT_DISTRIBUTE_URL`）。
pub const GET_PRODUCT_DEFAULT_DISTRIBUTE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/get_product_default_distribute";

/// 获取按商品自动分配的商品列表（对应 Java `Supplier.GET_PRODUCT_LIST_URL`）。
pub const GET_PRODUCT_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/supplier/relation/get_product_list";

/// 分配订单代发（对应 Java `Supplier.ASSIGN_DROPSHIP_URL`）。
pub const ASSIGN_DROPSHIP_URL: &str = "https://api.weixin.qq.com/channels/ec/order/dropship/assign";

/// 取消分配代发单（对应 Java `Supplier.CANCEL_DROPSHIP_URL`）。
pub const CANCEL_DROPSHIP_URL: &str = "https://api.weixin.qq.com/channels/ec/order/dropship/cancel";

/// 查询代发单详情（对应 Java `Supplier.GET_DROPSHIP_URL`）。
pub const GET_DROPSHIP_URL: &str = "https://api.weixin.qq.com/channels/ec/order/dropship/get";

/// 拉取代发单列表（对应 Java `Supplier.GET_DROPSHIP_LIST_URL`）。
pub const GET_DROPSHIP_LIST_URL: &str = "https://api.weixin.qq.com/channels/ec/order/dropship/list";

/// 搜索代发单（对应 Java `Supplier.SEARCH_DROPSHIP_URL`）。
pub const SEARCH_DROPSHIP_URL: &str = "https://api.weixin.qq.com/channels/ec/order/dropship/search";
