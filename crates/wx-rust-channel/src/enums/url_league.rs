//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 添加团长商品到橱窗（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_SUPPLIER_GOODS_URL`）。
pub const ADD_SUPPLIER_GOODS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/window/add";

/// 查询橱窗上团长商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_SUPPLIER_GOODS_URL`）。
pub const LIST_SUPPLIER_GOODS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/window/getall";

/// 从橱窗移除团长商品（对应 Java `WxChannelApiUrlConstants` 常量 `REMOVE_SUPPLIER_GOODS_URL`）。
pub const REMOVE_SUPPLIER_GOODS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/window/remove";

/// 查询橱窗上团长商品详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_GOODS_URL`）。
pub const GET_SUPPLIER_GOODS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/window/getdetail";

/// 获取达人橱窗授权链接（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_AUTH_URL`）。
pub const GET_SUPPLIER_AUTH_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/windowauth/get";

/// 获取达人橱窗授权状态（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_AUTH_STATUS_URL`）。
pub const GET_SUPPLIER_AUTH_STATUS_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/windowauth/status/get";

/// 获取团长账户余额（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_BALANCE_URL`）。
pub const GET_SUPPLIER_BALANCE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/funds/balance/get";

/// 获取资金流水详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_BALANCE_FLOW_DETAIL_URL`）。
pub const GET_SUPPLIER_BALANCE_FLOW_DETAIL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/funds/flowdetail/get";

/// 获取资金流水列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_BALANCE_FLOW_LIST_URL`）。
pub const GET_SUPPLIER_BALANCE_FLOW_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/funds/flowlist/get";

/// 获取合作商品详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_ITEM_URL`）。
pub const GET_SUPPLIER_ITEM_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/item/get";

/// 获取合作商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_ITEM_LIST_URL`）。
pub const GET_SUPPLIER_ITEM_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/item/list/get";

/// 获取佣金单详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_ORDER_URL`）。
pub const GET_SUPPLIER_ORDER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/order/get";

/// 获取佣金单列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_ORDER_LIST_URL`）。
pub const GET_SUPPLIER_ORDER_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/order/list/get";

/// 获取合作小店详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_SHOP_URL`）。
pub const GET_SUPPLIER_SHOP_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/shop/get";

/// 获取合作小店列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUPPLIER_SHOP_LIST_URL`）。
pub const GET_SUPPLIER_SHOP_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/headsupplier/shop/list/get";

/// 新增达人（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_PROMOTER_URL`）。
pub const ADD_PROMOTER_URL: &str = "https://api.weixin.qq.com/channels/ec/league/promoter/add";

/// 编辑达人（对应 Java `WxChannelApiUrlConstants` 常量 `EDIT_PROMOTER_URL`）。
pub const EDIT_PROMOTER_URL: &str = "https://api.weixin.qq.com/channels/ec/league/promoter/upd";

/// 删除达人（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_PROMOTER_URL`）。
pub const DELETE_PROMOTER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/promoter/delete";

/// 获取达人详情信息（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PROMOTER_URL`）。
pub const GET_PROMOTER_URL: &str = "https://api.weixin.qq.com/channels/ec/league/promoter/get";

/// 拉取商店达人列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PROMOTER_LIST_URL`）。
pub const GET_PROMOTER_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/promoter/list/get";

/// 批量新增联盟商品（对应 Java `WxChannelApiUrlConstants` 常量 `BATCH_ADD_LEAGUE_ITEM_URL`）。
pub const BATCH_ADD_LEAGUE_ITEM_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/item/batchadd";

/// 更新联盟商品信息（对应 Java `WxChannelApiUrlConstants` 常量 `UPDATE_LEAGUE_ITEM_URL`）。
pub const UPDATE_LEAGUE_ITEM_URL: &str = "https://api.weixin.qq.com/channels/ec/league/item/upd";

/// 删除联盟商品（对应 Java `WxChannelApiUrlConstants` 常量 `DELETE_LEAGUE_ITEM_URL`）。
pub const DELETE_LEAGUE_ITEM_URL: &str = "https://api.weixin.qq.com/channels/ec/league/item/delete";

/// 拉取联盟商品详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEAGUE_ITEM_URL`）。
pub const GET_LEAGUE_ITEM_URL: &str = "https://api.weixin.qq.com/channels/ec/league/item/get";

/// 拉取联盟商品推广列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_LEAGUE_ITEM_LIST_URL`）。
pub const GET_LEAGUE_ITEM_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/league/item/list/get";
