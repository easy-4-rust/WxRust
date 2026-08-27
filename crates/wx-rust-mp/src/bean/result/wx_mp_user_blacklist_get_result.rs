//! 拉黑用户列表结果。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.result.WxMpUserBlacklistGetResult`。
//! 线格式由 `WxUserBlacklistGetResultGsonAdapter` 决定：`data.openid` 数组。

use serde::{Deserialize, Serialize};

/// 拉黑用户列表结果。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WxMpUserBlacklistGetResult {
    /// 拉黑用户总数。
    #[serde(default = "default_minus_one")]
    pub total: i32,
    /// 拉取的用户数量。
    #[serde(default = "default_minus_one")]
    pub count: i32,
    /// 拉黑用户 openid 列表（来自 `data.openid`）。
    #[serde(default)]
    pub openid_list: Vec<String>,
    /// 拉取列表的最后一个用户的 openid。
    #[serde(rename = "next_openid", default)]
    pub next_openid: String,
}

/// 默认值 -1（对应 Java 字段初始化 `= -1`）。
fn default_minus_one() -> i32 {
    -1
}

impl WxMpUserBlacklistGetResult {
    /// 从 JSON 构建（对应 Java `fromJson` + `WxUserBlacklistGetResultGsonAdapter`）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|e| format!("拉黑用户列表解析失败: {e}"))?;
        let mut result = Self {
            total: value
                .get("total")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32)
                .unwrap_or(-1),
            count: value
                .get("count")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32)
                .unwrap_or(-1),
            openid_list: Vec::new(),
            next_openid: value
                .get("next_openid")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
        };
        if let Some(data) = value.get("data").and_then(|v| v.as_object())
            && let Some(openids) = data.get("openid").and_then(|v| v.as_array())
        {
            result.openid_list = openids
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }
        Ok(result)
    }
}
