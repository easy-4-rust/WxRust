//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 添加分类关联的商品（对应 Java `WxChannelApiUrlConstants` 常量 `ADD_TREE_PRODUCT_URL`）。
pub const ADD_TREE_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/classification/tree/product/add";

/// 删除分类关联的商品（对应 Java `WxChannelApiUrlConstants` 常量 `DEL_TREE_PRODUCT_URL`）。
pub const DEL_TREE_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/classification/tree/product/del";

/// 获取分类关联的商品ID列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_TREE_PRODUCT_URL`）。
pub const LIST_TREE_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/classification/tree/product/get";

/// 设置展示在店铺主页的商品分类（对应 Java `WxChannelApiUrlConstants` 常量 `SET_SHOW_TREE_URL`）。
pub const SET_SHOW_TREE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/classification/tree/set";

/// 获取在店铺主页展示的商品分类（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SHOW_TREE_URL`）。
pub const GET_SHOW_TREE_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/classification/tree/get";

/// 获取主页展示商品列表（对应 Java `WxChannelApiUrlConstants` 常量 `LIST_WINDOW_PRODUCT_URL`）。
pub const LIST_WINDOW_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/window/product/list/get";

/// 重新排序主页展示商品（对应 Java `WxChannelApiUrlConstants` 常量 `REORDER_WINDOW_PRODUCT_URL`）。
pub const REORDER_WINDOW_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/window/product/reorder";

/// 隐藏小店主页商品（对应 Java `WxChannelApiUrlConstants` 常量 `HIDE_WINDOW_PRODUCT_URL`）。
pub const HIDE_WINDOW_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/window/product/hide";

/// 置顶小店主页商品（对应 Java `WxChannelApiUrlConstants` 常量 `TOP_WINDOW_PRODUCT_URL`）。
pub const TOP_WINDOW_PRODUCT_URL: &str =
    "https://api.weixin.qq.com/channels/ec/store/window/product/settop";

/// 提交主页背景图申请（对应 Java `WxChannelApiUrlConstants` 常量 `APPLY_BACKGROUND_URL`）。
pub const APPLY_BACKGROUND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/background/apply/submit";

/// 查询主页背景图（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BACKGROUND_URL`）。
pub const GET_BACKGROUND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/background/get";

/// 撤销主页背景图申请（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_BACKGROUND_URL`）。
pub const CANCEL_BACKGROUND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/background/apply/cancel";

/// 清空主页背景图并撤销流程中的申请（对应 Java `WxChannelApiUrlConstants` 常量 `REMOVE_BACKGROUND_URL`）。
pub const REMOVE_BACKGROUND_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/background/remove";

/// 提交精选展示位申请（对应 Java `WxChannelApiUrlConstants` 常量 `APPLY_BANNER_URL`）。
pub const APPLY_BANNER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/banner/apply/submit";

/// 查询精选展示位（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BANNER_URL`）。
pub const GET_BANNER_URL: &str = "https://api.weixin.qq.com/channels/ec/basics/homepage/banner/get";

/// 撤销精选展示位申请（对应 Java `WxChannelApiUrlConstants` 常量 `CANCEL_BANNER_URL`）。
pub const CANCEL_BANNER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/banner/apply/cancel";

/// 清空精选展示位并撤销流程中的申请（对应 Java `WxChannelApiUrlConstants` 常量 `REMOVE_BANNER_URL`）。
pub const REMOVE_BANNER_URL: &str =
    "https://api.weixin.qq.com/channels/ec/basics/homepage/banner/remove";
