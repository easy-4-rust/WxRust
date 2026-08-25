//! 视频号小店限时抢购接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!
//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理。

/// 添加限时抢购任务（对应 Java `ADD_LIMIT_TASK_URL`）。
pub const ADD_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/add";

/// 拉取限时抢购任务列表（对应 Java `LIST_LIMIT_TASK_URL`）。
pub const LIST_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/list/get";

/// 停止限时抢购任务（对应 Java `STOP_LIMIT_TASK_URL`）。
pub const STOP_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/stop";

/// 删除限时抢购任务（对应 Java `DELETE_LIMIT_TASK_URL`）。
pub const DELETE_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/delete";

/// 更新限时抢购任务（对应 Java `UPDATE_LIMIT_TASK_URL`）。
pub const UPDATE_LIMIT_TASK_URL: &str =
    "https://api.weixin.qq.com/channels/ec/product/limiteddiscounttask/update";
