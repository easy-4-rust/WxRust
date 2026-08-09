//! 用户留存分析数据。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.analysis.WxMaRetainInfo`。
//! 线格式由 `WxMaRetainInfoGsonAdapter` 决定：`ref_date` + `visit_uv_new`/
//! `visit_uv`（`[{key, value}]` 数组映射为 `{key -> value}` 的 Map）。

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

/// 用户留存分析数据（对应 Java `WxMaRetainInfo`）。
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct WxMaRetainInfo {
    /// 日期。
    pub ref_date: String,
    /// 新增用户留存。
    pub visit_uv_new: HashMap<i32, i32>,
    /// 活跃用户留存。
    pub visit_uv: HashMap<i32, i32>,
}

impl<'de> Deserialize<'de> for WxMaRetainInfo {
    /// 对应 Java `WxMaRetainInfoGsonAdapter.deserialize`。
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let v = serde_json::Value::deserialize(d)?;
        let obj = v
            .as_object()
            .ok_or_else(|| serde::de::Error::custom("WxMaRetainInfo 应为 JSON 对象"))?;
        Ok(WxMaRetainInfo {
            ref_date: obj
                .get("ref_date")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            visit_uv_new: serde_json::from_value(
                obj.get("visit_uv_new")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(Vec::new())),
            )
            .unwrap_or_default(),
            visit_uv: serde_json::from_value(
                obj.get("visit_uv")
                    .cloned()
                    .unwrap_or(serde_json::Value::Array(Vec::new())),
            )
            .unwrap_or_default(),
        })
    }
}

impl WxMaRetainInfo {
    /// 从 JSON 构建（对应 Java `fromJson`，adapter 线格式）。
    pub fn from_json(json: &str) -> Result<Self, String> {
        serde_json::from_str(json).map_err(|e| format!("留存分析解析失败: {e}"))
    }
}
