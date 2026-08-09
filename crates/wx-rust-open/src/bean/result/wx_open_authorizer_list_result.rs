//! 拉取所有已授权的帐号信息的结果。
//!
//! 对应 Java `me.chanjar.weixin.open.bean.result.WxOpenAuthorizerListResult`。
//! 由 `WxOpenAuthorizerListResultGsonAdapter` 驱动解析
//! （`total_count`/`list` 键，list 元素展开为 authorizer_appid/
//! refresh_token/auth_time 三键 Map），与字段名直映不同，故人工迁移。

use std::collections::HashMap;

/// 拉取所有已授权的帐号信息的结果。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOpenAuthorizerListResult {
    /// 授权的帐号总数。
    #[serde(rename = "total_count", default)]
    pub total_count: Option<i32>,
    /// 授权的帐号列表，每项含 `authorizer_appid`/`refresh_token`/`auth_time` 键
    /// （对应 Java adapter 的 Map 展开语义）。
    #[serde(rename = "list", default)]
    pub list: Option<Vec<HashMap<String, String>>>,
}
