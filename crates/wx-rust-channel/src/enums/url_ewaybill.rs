//! 视频号小店电子面单接口地址常量（对应 Java `WxChannelApiUrlConstants.Ewaybill`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 获取标准面单模板配置（对应 Java `Ewaybill.GET_TEMPLATE_CONFIG_URL`）。
pub const GET_TEMPLATE_CONFIG_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/config";

/// 创建商家面单模板（对应 Java `Ewaybill.CREATE_TEMPLATE_URL`）。
pub const CREATE_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/create";

/// 删除商家面单模板（对应 Java `Ewaybill.DELETE_TEMPLATE_URL`）。
pub const DELETE_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/delete";

/// 更新商家面单模板（对应 Java `Ewaybill.UPDATE_TEMPLATE_URL`）。
pub const UPDATE_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/update";

/// 查询标准模板信息（对应 Java `Ewaybill.GET_TEMPLATE_URL`）。
pub const GET_TEMPLATE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/get";

/// 按模板 ID 查询商家模板（对应 Java `Ewaybill.GET_TEMPLATE_BY_ID_URL`）。
pub const GET_TEMPLATE_BY_ID_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/template/getbyid";

/// 查询已开通电子面单的网点和账号（对应 Java `Ewaybill.GET_ACCOUNT_URL`）。
pub const GET_ACCOUNT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/account/get";

/// 查询已开通电子面单的快递公司（对应 Java `Ewaybill.GET_DELIVERY_LIST_URL`）。
pub const GET_DELIVERY_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/delivery/get";

/// 预取电子面单号（对应 Java `Ewaybill.PRE_CREATE_ORDER_URL`）。
pub const PRE_CREATE_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/precreate";

/// 获取电子面单号（对应 Java `Ewaybill.CREATE_ORDER_URL`）。
pub const CREATE_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/create";

/// 追加电子面单子件（对应 Java `Ewaybill.ADD_SUB_ORDER_URL`）。
pub const ADD_SUB_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/addsuborder";

/// 取消电子面单下单（对应 Java `Ewaybill.CANCEL_ORDER_URL`）。
pub const CANCEL_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/cancel";

/// 查询电子面单详情（对应 Java `Ewaybill.GET_ORDER_URL`）。
pub const GET_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/get";

/// 获取打印报文（对应 Java `Ewaybill.GET_PRINT_CONTENT_URL`）。
pub const GET_PRINT_CONTENT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/print/get";

/// 通知单个运单打印成功（对应 Java `Ewaybill.PRINT_ORDER_URL`）。
pub const PRINT_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/print";

/// 批量通知运单打印成功（对应 Java `Ewaybill.BATCH_PRINT_ORDER_URL`）。
pub const BATCH_PRINT_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/logistics/ewaybill/biz/order/batchprint";
